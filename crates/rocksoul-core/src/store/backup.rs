use std::{
    fs::{self, File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{Connection, SqliteConnection, sqlite::SqliteConnectOptions};
use uuid::Uuid;

use super::Store;
use crate::StoreError;

const BACKUP_FORMAT_VERSION: u32 = 1;
const BUSY_RETRY_LIMIT: usize = 3;
const BUSY_RETRY_DELAY: Duration = Duration::from_millis(150);

/// Public, non-secret description of a finalized local database snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifest {
    pub format_version: u32,
    pub schema_version: i64,
    pub app_version: String,
    pub created_at: String,
    pub database_file: String,
    pub integrity_result: String,
    pub sha256: String,
    pub database_bytes: u64,
}

/// Paths and manifest returned only after the staged snapshot is verified and
/// the containing directory has been atomically finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackupResult {
    pub backup_directory: PathBuf,
    pub database_path: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest: BackupManifest,
}

struct StagingDirectory {
    path: PathBuf,
    committed: bool,
}

impl StagingDirectory {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            committed: false,
        }
    }

    fn commit(&mut self) {
        self.committed = true;
    }
}

impl Drop for StagingDirectory {
    fn drop(&mut self) {
        if !self.committed {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

impl Store {
    /// Create a WAL-safe SQLite snapshot in a new child directory of
    /// destination_directory. The final directory appears only after the
    /// database passes integrity and foreign-key checks and its hash/manifest
    /// have been written and synced.
    ///
    /// VACUUM INTO creates a consistent snapshot without copying the live main
    /// database file or separating committed WAL pages from that database.
    pub async fn create_backup(
        &self,
        destination_directory: &Path,
    ) -> Result<BackupResult, StoreError> {
        fs::create_dir_all(destination_directory)?;
        let destination_directory = destination_directory.canonicalize()?;
        if !destination_directory.is_dir() {
            return Err(StoreError::Backup(
                "The backup destination must be a directory.".to_string(),
            ));
        }

        let backup_name = format!(
            "rocksoul-backup-{}-{}",
            Utc::now().format("%Y%m%dT%H%M%SZ"),
            Uuid::new_v4()
        );
        let staging_path = destination_directory.join(format!(".{backup_name}.partial"));
        let final_path = destination_directory.join(&backup_name);
        fs::create_dir(&staging_path)?;
        let mut staging = StagingDirectory::new(staging_path.clone());

        let database_path = staging_path.join("database.sqlite3");
        let database_path_for_sql = database_path.to_string_lossy().into_owned();

        // VACUUM INTO is a single SQLite snapshot operation. A bounded retry
        // handles transient SQLITE_BUSY/SQLITE_LOCKED responses; the configured
        // SQLite busy timeout still applies inside each attempt.
        let mut connection = self.pool.acquire().await?;
        let mut snapshot_created = false;
        for attempt in 0..BUSY_RETRY_LIMIT {
            match sqlx::query("VACUUM INTO ?")
                .bind(&database_path_for_sql)
                .execute(&mut *connection)
                .await
            {
                Ok(_) => {
                    snapshot_created = true;
                    break;
                }
                Err(error) if should_retry_busy(&error, attempt) => {
                    tokio::time::sleep(BUSY_RETRY_DELAY * (attempt as u32 + 1)).await;
                }
                Err(error) => return Err(error.into()),
            }
        }
        drop(connection);

        if !snapshot_created {
            return Err(StoreError::Backup(
                "The database remained busy after the bounded backup retries.".to_string(),
            ));
        }

        let database_bytes = fs::metadata(&database_path)?.len();
        OpenOptions::new()
            .write(true)
            .open(&database_path)?
            .sync_all()?;

        let mut snapshot = SqliteConnection::connect_with(
            &SqliteConnectOptions::new()
                .filename(&database_path)
                .read_only(true)
                .create_if_missing(false)
                .foreign_keys(true)
                .busy_timeout(Duration::from_secs(5)),
        )
        .await?;

        let integrity_result: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&mut snapshot)
            .await?;
        if integrity_result != "ok" {
            return Err(StoreError::Backup(format!(
                "SQLite integrity_check rejected the staged snapshot: {integrity_result}"
            )));
        }

        let foreign_key_violation = sqlx::query("PRAGMA foreign_key_check")
            .fetch_optional(&mut snapshot)
            .await?;
        if foreign_key_violation.is_some() {
            return Err(StoreError::Backup(
                "SQLite foreign_key_check found a violation in the staged snapshot.".to_string(),
            ));
        }

        let schema_version: i64 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(version), 0) FROM _sqlx_migrations WHERE success = 1",
        )
        .fetch_one(&mut snapshot)
        .await?;
        snapshot.close().await?;

        let database_path_for_hash = database_path.clone();
        let sha256 = tokio::task::spawn_blocking(move || sha256_file(&database_path_for_hash))
            .await
            .map_err(|error| StoreError::Task(error.to_string()))??;

        let manifest = BackupManifest {
            format_version: BACKUP_FORMAT_VERSION,
            schema_version,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: Utc::now().to_rfc3339(),
            database_file: "database.sqlite3".to_string(),
            integrity_result,
            sha256,
            database_bytes,
        };

        let manifest_path = staging_path.join("manifest.json");
        let manifest_bytes = serde_json::to_vec_pretty(&manifest)
            .map_err(|error| StoreError::Backup(format!("Could not encode manifest: {error}")))?;
        fs::write(&manifest_path, manifest_bytes)?;
        OpenOptions::new()
            .write(true)
            .open(&manifest_path)?
            .sync_all()?;

        // The database and manifest are finalized together by a same-volume
        // directory rename. Consumers must ignore hidden *.partial directories.
        fs::rename(&staging_path, &final_path)?;
        staging.commit();

        Ok(BackupResult {
            database_path: final_path.join("database.sqlite3"),
            manifest_path: final_path.join("manifest.json"),
            backup_directory: final_path,
            manifest,
        })
    }
}

fn should_retry_busy(error: &sqlx::Error, attempt: usize) -> bool {
    is_sqlite_busy(error) && attempt + 1 < BUSY_RETRY_LIMIT
}

fn is_sqlite_busy(error: &sqlx::Error) -> bool {
    let sqlx::Error::Database(database_error) = error else {
        return false;
    };

    matches!(
        database_error.code().as_deref(),
        Some("5" | "6" | "SQLITE_BUSY" | "SQLITE_LOCKED")
    ) || {
        let message = database_error.message().to_ascii_lowercase();
        message.contains("database is locked") || message.contains("database is busy")
    }
}

fn sha256_file(path: &Path) -> Result<String, StoreError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs, time::Duration};

    use serde_json::from_slice;
    use sha2::{Digest, Sha256};
    use sqlx::{
        Connection, SqliteConnection,
        sqlite::{SqliteConnectOptions, SqliteJournalMode},
    };
    use tempfile::tempdir;

    use super::{BackupManifest, Store};
    use crate::StoreError;

    #[tokio::test]
    async fn wal_backup_has_a_verified_manifest_and_restores_committed_rows()
    -> Result<(), Box<dyn Error>> {
        let temporary = tempdir()?;
        let workspace = temporary.path().join("workspace");
        fs::create_dir_all(&workspace)?;
        fs::write(workspace.join("README.md"), "backup fixture")?;

        let database = temporary.path().join("rocksoul.sqlite3");
        let store = Store::connect(&database).await?;
        let project = store
            .create_project("Backup fixture", workspace.to_str().unwrap_or_default())
            .await?;
        let mission = store
            .create_mission(
                &project.id,
                "Verify online backup",
                "All committed rows must be present in the verified snapshot.",
            )
            .await?;

        let mut source_connection = store.pool.acquire().await?;
        let journal_mode: String = sqlx::query_scalar("PRAGMA journal_mode")
            .fetch_one(&mut *source_connection)
            .await?;
        assert_eq!(journal_mode.to_ascii_lowercase(), "wal");
        drop(source_connection);

        let backup = store
            .create_backup(&temporary.path().join("backups"))
            .await?;

        assert!(backup.database_path.is_file());
        assert!(backup.manifest_path.is_file());
        assert_eq!(backup.manifest.format_version, 1);
        assert_eq!(backup.manifest.schema_version, 2);
        assert_eq!(backup.manifest.integrity_result, "ok");
        assert_eq!(backup.manifest.app_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(backup.manifest.database_file, "database.sqlite3");
        assert!(!backup.manifest.created_at.is_empty());
        assert_eq!(backup.manifest.sha256.len(), 64);
        let expected_hash = format!("{:x}", Sha256::digest(fs::read(&backup.database_path)?));
        assert_eq!(backup.manifest.sha256, expected_hash);
        assert_eq!(
            fs::metadata(&backup.database_path)?.len(),
            backup.manifest.database_bytes
        );

        let manifest: BackupManifest = from_slice(&fs::read(&backup.manifest_path)?)?;
        assert_eq!(manifest, backup.manifest);
        let bundle_entries = fs::read_dir(&backup.backup_directory)?
            .map(|entry| entry.map(|entry| entry.file_name()))
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            bundle_entries.len(),
            2,
            "only the database and manifest are finalized"
        );
        assert!(
            !bundle_entries
                .iter()
                .any(|name| name.to_string_lossy().ends_with(".partial"))
        );

        // Open the snapshot as a separate local store and prove that the
        // project and mission committed to the source WAL are in the snapshot.
        let restored = Store::connect(&backup.database_path).await?;
        let mut restored_connection = restored.pool.acquire().await?;
        let restored_integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&mut *restored_connection)
            .await?;
        assert_eq!(restored_integrity, "ok");
        drop(restored_connection);

        let projects = restored.list_projects().await?;
        let missions = restored.list_missions(&project.id).await?;
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, project.id);
        assert_eq!(missions.len(), 1);
        assert_eq!(missions[0].id, mission.id);
        assert_eq!(missions[0].title, "Verify online backup");

        Ok(())
    }

    #[tokio::test]
    async fn sqlite_busy_errors_are_detected_and_retry_budget_is_bounded()
    -> Result<(), Box<dyn Error>> {
        let temporary = tempdir()?;
        let database = temporary.path().join("busy.sqlite3");
        let options = SqliteConnectOptions::new()
            .filename(&database)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Delete);
        let mut lock_owner = SqliteConnection::connect_with(&options).await?;
        sqlx::query("CREATE TABLE lock_fixture (id INTEGER PRIMARY KEY)")
            .execute(&mut lock_owner)
            .await?;
        // Open the contender before taking the exclusive lock. Opening a
        // SQLite connection can itself touch journal metadata, so doing this
        // afterward would test connection setup rather than the retry path.
        let contender_options = SqliteConnectOptions::new()
            .filename(&database)
            .create_if_missing(false)
            .journal_mode(SqliteJournalMode::Delete)
            .busy_timeout(Duration::from_millis(10));
        let mut contender = SqliteConnection::connect_with(&contender_options).await?;

        sqlx::query("BEGIN EXCLUSIVE")
            .execute(&mut lock_owner)
            .await?;

        let busy_error = sqlx::query("BEGIN IMMEDIATE")
            .execute(&mut contender)
            .await
            .expect_err("the exclusive SQLite lock must reject a second writer");

        assert!(super::is_sqlite_busy(&busy_error));
        assert!(super::should_retry_busy(&busy_error, 0));
        assert!(super::should_retry_busy(&busy_error, 1));
        assert!(!super::should_retry_busy(&busy_error, 2));

        contender.close().await?;
        sqlx::query("ROLLBACK").execute(&mut lock_owner).await?;
        lock_owner.close().await?;
        Ok(())
    }

    #[test]
    fn abandoned_staging_directory_is_removed() -> Result<(), Box<dyn Error>> {
        let temporary = tempdir()?;
        let partial = temporary.path().join(".backup.partial");
        fs::create_dir(&partial)?;
        {
            let _staging = super::StagingDirectory::new(partial.clone());
        }
        assert!(!partial.exists());
        Ok(())
    }

    #[tokio::test]
    async fn invalid_backup_destination_leaves_live_database_usable() -> Result<(), Box<dyn Error>>
    {
        let temporary = tempdir()?;
        let workspace = temporary.path().join("workspace");
        fs::create_dir_all(&workspace)?;
        let store = Store::connect(&temporary.path().join("rocksoul.sqlite3")).await?;
        let project = store
            .create_project("Still usable", workspace.to_str().unwrap_or_default())
            .await?;

        let invalid_destination = temporary.path().join("not-a-directory");
        fs::write(&invalid_destination, "not a directory")?;
        let result = store.create_backup(&invalid_destination).await;

        assert!(matches!(result, Err(StoreError::Io(_))));
        let projects = store.list_projects().await?;
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, project.id);
        Ok(())
    }
}
