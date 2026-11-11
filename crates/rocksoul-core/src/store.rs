use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use chrono::Utc;
use ignore::WalkBuilder;
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::{
    Sqlite, SqlitePool, Transaction,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use uuid::Uuid;

mod backup;

pub use backup::{BackupManifest, BackupResult};

use crate::{Artifact, Evidence, InspectionResult, Mission, Project, StoreError};

const MAX_INVENTORY_FILES: usize = 300;
const MAX_INVENTORY_ENTRIES: usize = 2_000;
const MAX_WALK_DEPTH: usize = 12;
const MAX_WARNINGS: usize = 20;

#[derive(Clone)]
pub struct Store {
    pool: SqlitePool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InventoryFile {
    path: String,
    bytes: u64,
    extension: String,
}

impl Store {
    pub async fn connect(database_path: &Path) -> Result<Self, StoreError> {
        if let Some(parent) = database_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let options = SqliteConnectOptions::new()
            .filename(database_path)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(5));

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>, StoreError> {
        Ok(sqlx::query_as::<_, Project>(
            "SELECT id, name, root_path, created_at, updated_at FROM projects ORDER BY updated_at DESC",
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn create_project(&self, name: &str, root_path: &str) -> Result<Project, StoreError> {
        let name = validate_text("Project name", name, 80)?;
        let supplied_path = PathBuf::from(root_path.trim());
        let canonical_root = supplied_path.canonicalize().map_err(|error| {
            StoreError::ProjectDirectory(format!("could not open the selected folder ({error})"))
        })?;

        if !canonical_root.is_dir() {
            return Err(StoreError::ProjectDirectory(
                "the selected path is not a directory".to_string(),
            ));
        }

        let timestamp = now();
        let project = Project {
            id: Uuid::new_v4().to_string(),
            name,
            root_path: canonical_root.to_string_lossy().into_owned(),
            created_at: timestamp.clone(),
            updated_at: timestamp,
        };

        // The insert is the first statement in the transaction, so SQLite
        // serializes competing writers before the duplicate-name check.
        let mut transaction = self.pool.begin().await?;
        sqlx::query(
            "INSERT INTO projects (id, name, root_path, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&project.id)
        .bind(&project.name)
        .bind(&project.root_path)
        .bind(&project.created_at)
        .bind(&project.updated_at)
        .execute(&mut *transaction)
        .await?;

        if project_name_conflicts(&mut transaction, &project.name, Some(&project.id)).await? {
            transaction.rollback().await?;
            return Err(StoreError::ProjectNameConflict);
        }

        transaction.commit().await?;
        Ok(project)
    }

    /// Rename a project without changing its stable ID, canonical workspace
    /// path, creation time, or any mission/artifact/evidence relationships.
    pub async fn rename_project(
        &self,
        project_id: &str,
        name: &str,
    ) -> Result<Project, StoreError> {
        let name = validate_text("Project name", name, 80)?;
        let timestamp = now();
        let mut transaction = self.pool.begin().await?;

        // Updating first acquires SQLite's writer reservation before checking
        // duplicates, preventing concurrent create/rename operations from
        // passing the same availability check.
        let changed = sqlx::query("UPDATE projects SET name = ?, updated_at = ? WHERE id = ?")
            .bind(&name)
            .bind(&timestamp)
            .bind(project_id)
            .execute(&mut *transaction)
            .await?
            .rows_affected();

        if changed != 1 {
            transaction.rollback().await?;
            return Err(StoreError::NotFound);
        }

        if project_name_conflicts(&mut transaction, &name, Some(project_id)).await? {
            transaction.rollback().await?;
            return Err(StoreError::ProjectNameConflict);
        }

        let project = sqlx::query_as::<_, Project>(
            "SELECT id, name, root_path, created_at, updated_at FROM projects WHERE id = ?",
        )
        .bind(project_id)
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;
        Ok(project)
    }

    pub async fn list_missions(&self, project_id: &str) -> Result<Vec<Mission>, StoreError> {
        Ok(sqlx::query_as::<_, Mission>(
            "SELECT id, project_id, title, objective, status, created_at, updated_at FROM missions WHERE project_id = ? ORDER BY updated_at DESC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn create_mission(
        &self,
        project_id: &str,
        title: &str,
        objective: &str,
    ) -> Result<Mission, StoreError> {
        let title = validate_text("Mission title", title, 120)?;
        let objective = validate_text("Mission objective", objective, 2_000)?;

        let project_exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects WHERE id = ?")
            .bind(project_id)
            .fetch_one(&self.pool)
            .await?;

        if project_exists == 0 {
            return Err(StoreError::NotFound);
        }

        let timestamp = now();
        let mission = Mission {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            title,
            objective,
            status: "draft".to_string(),
            created_at: timestamp.clone(),
            updated_at: timestamp,
        };

        sqlx::query(
            "INSERT INTO missions (id, project_id, title, objective, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&mission.id)
        .bind(&mission.project_id)
        .bind(&mission.title)
        .bind(&mission.objective)
        .bind(&mission.status)
        .bind(&mission.created_at)
        .bind(&mission.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(mission)
    }

    pub async fn list_artifacts(&self, mission_id: &str) -> Result<Vec<Artifact>, StoreError> {
        Ok(sqlx::query_as::<_, Artifact>(
            "SELECT id, mission_id, kind, title, content, sha256, created_at FROM artifacts WHERE mission_id = ? ORDER BY created_at DESC",
        )
        .bind(mission_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn list_evidence(&self, mission_id: &str) -> Result<Vec<Evidence>, StoreError> {
        Ok(sqlx::query_as::<_, Evidence>(
            "SELECT id, mission_id, artifact_id, kind, summary, source_path, details_json, created_at FROM evidence WHERE mission_id = ? ORDER BY created_at DESC",
        )
        .bind(mission_id)
        .fetch_all(&self.pool)
        .await?)
    }

    /// Creates a bounded metadata-only inventory. No file content is opened and
    /// the selected workspace is never modified. Artifact and evidence writes
    /// are committed together in the local Rocksoul database.
    pub async fn inspect_workspace(
        &self,
        project_id: &str,
        mission_id: &str,
    ) -> Result<InspectionResult, StoreError> {
        let project = sqlx::query_as::<_, Project>(
            "SELECT id, name, root_path, created_at, updated_at FROM projects WHERE id = ?",
        )
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(StoreError::NotFound)?;

        let mission = sqlx::query_as::<_, Mission>(
            "SELECT id, project_id, title, objective, status, created_at, updated_at FROM missions WHERE id = ? AND project_id = ?",
        )
        .bind(mission_id)
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(StoreError::NotFound)?;

        if mission.status != "draft" && mission.status != "in_progress" {
            return Err(StoreError::InvalidInput(
                "Only a draft or in-progress mission can run the Workspace Scout.".to_string(),
            ));
        }

        let root = PathBuf::from(&project.root_path)
            .canonicalize()
            .map_err(|error| {
                StoreError::ProjectDirectory(format!(
                    "the project folder is no longer available ({error})"
                ))
            })?;

        if !root.is_dir() {
            return Err(StoreError::ProjectDirectory(
                "the project folder is no longer a directory".to_string(),
            ));
        }

        let inventory_root = root.clone();
        let (files, truncated, warnings) =
            tokio::task::spawn_blocking(move || collect_inventory(&inventory_root))
                .await
                .map_err(|error| StoreError::Task(error.to_string()))?;
        let mut extensions = BTreeMap::<String, usize>::new();
        for file in &files {
            *extensions.entry(file.extension.clone()).or_default() += 1;
        }

        let content = serde_json::to_string_pretty(&json!({
            "schemaVersion": 1,
            "project": project.name,
            "mission": mission.title,
            "objective": mission.objective,
            "root": root.to_string_lossy(),
            "mode": "read-only metadata inventory; source/document/binary contents were not opened; ignore rules were read",
            "fileCount": files.len(),
            "truncated": truncated,
            "extensions": extensions,
            "files": files,
            "warnings": warnings,
            "limits": {
                "maxFiles": MAX_INVENTORY_FILES,
                "maxEntries": MAX_INVENTORY_ENTRIES,
                "maxDepth": MAX_WALK_DEPTH,
                "maxWarnings": MAX_WARNINGS
            }
        }))
        .map_err(|error| {
            StoreError::InvalidInput(format!("could not serialize inventory: {error}"))
        })?;

        let digest = format!("{:x}", Sha256::digest(content.as_bytes()));
        let timestamp = now();

        let artifact = Artifact {
            id: Uuid::new_v4().to_string(),
            mission_id: mission.id.clone(),
            kind: "workspace_inventory".to_string(),
            title: "Workspace inventory".to_string(),
            content,
            sha256: digest,
            created_at: timestamp.clone(),
        };

        let evidence = Evidence {
            id: Uuid::new_v4().to_string(),
            mission_id: mission.id.clone(),
            artifact_id: Some(artifact.id.clone()),
            kind: "read_only_inventory".to_string(),
            summary: format!(
                "Inventoried {} file entries. Source/document/binary contents were not opened; Git ignore rules were consulted and no workspace files were modified.",
                files.len()
            ),
            source_path: Some(root.to_string_lossy().into_owned()),
            details_json: json!({
                "fileCount": files.len(),
                "truncated": truncated,
                "maxFiles": MAX_INVENTORY_FILES,
                "maxEntries": MAX_INVENTORY_ENTRIES,
                "maxDepth": MAX_WALK_DEPTH,
                "readSourceDocumentBinaryContents": false,
                "readIgnoreRules": true,
                "modifiedWorkspace": false
            })
            .to_string(),
            created_at: timestamp.clone(),
        };

        let mut transaction = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO artifacts (id, mission_id, kind, title, content, sha256, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&artifact.id)
        .bind(&artifact.mission_id)
        .bind(&artifact.kind)
        .bind(&artifact.title)
        .bind(&artifact.content)
        .bind(&artifact.sha256)
        .bind(&artifact.created_at)
        .execute(&mut *transaction)
        .await?;

        sqlx::query(
            "INSERT INTO evidence (id, mission_id, artifact_id, kind, summary, source_path, details_json, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&evidence.id)
        .bind(&evidence.mission_id)
        .bind(&evidence.artifact_id)
        .bind(&evidence.kind)
        .bind(&evidence.summary)
        .bind(&evidence.source_path)
        .bind(&evidence.details_json)
        .bind(&evidence.created_at)
        .execute(&mut *transaction)
        .await?;

        sqlx::query("UPDATE missions SET status = 'review', updated_at = ? WHERE id = ?")
            .bind(&timestamp)
            .bind(&mission.id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(InspectionResult {
            artifact,
            evidence,
            file_count: files.len(),
            truncated,
        })
    }

    /// Records a human review decision. A mission can only be approved after
    /// a successful inventory created both an artifact and an evidence receipt.
    pub async fn approve_mission(&self, mission_id: &str) -> Result<Mission, StoreError> {
        let mut transaction = self.pool.begin().await?;

        let mission = sqlx::query_as::<_, Mission>(
            "SELECT id, project_id, title, objective, status, created_at, updated_at
             FROM missions WHERE id = ?",
        )
        .bind(mission_id)
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(StoreError::NotFound)?;

        if mission.status != "review" {
            return Err(StoreError::InvalidInput(
                "Only a mission in review can be approved.".to_string(),
            ));
        }

        let artifact_id: Option<String> = sqlx::query_scalar(
            "SELECT id FROM artifacts WHERE mission_id = ? AND kind = 'workspace_inventory' ORDER BY created_at DESC LIMIT 1",
        )
        .bind(mission_id)
        .fetch_optional(&mut *transaction)
        .await?;

        let evidence_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM evidence WHERE mission_id = ? AND kind = 'read_only_inventory'",
        )
        .bind(mission_id)
        .fetch_one(&mut *transaction)
        .await?;

        if artifact_id.is_none() || evidence_count < 1 {
            return Err(StoreError::InvalidInput(
                "Approval blocked: the mission has no saved artifact and evidence receipt."
                    .to_string(),
            ));
        }

        let timestamp = now();
        let approval = Evidence {
            id: Uuid::new_v4().to_string(),
            mission_id: mission_id.to_string(),
            artifact_id: artifact_id.clone(),
            kind: "human_approval".to_string(),
            summary: "User reviewed and approved the workspace inventory.".to_string(),
            source_path: None,
            details_json: json!({
                "decision": "approved",
                "artifactId": artifact_id,
                "evidenceCountAtApproval": evidence_count
            })
            .to_string(),
            created_at: timestamp.clone(),
        };

        sqlx::query(
            "INSERT INTO evidence
             (id, mission_id, artifact_id, kind, summary, source_path, details_json, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&approval.id)
        .bind(&approval.mission_id)
        .bind(&approval.artifact_id)
        .bind(&approval.kind)
        .bind(&approval.summary)
        .bind(&approval.source_path)
        .bind(&approval.details_json)
        .bind(&approval.created_at)
        .execute(&mut *transaction)
        .await?;

        let changed = sqlx::query(
            "UPDATE missions SET status = 'completed', updated_at = ?
             WHERE id = ? AND status = 'review'",
        )
        .bind(&timestamp)
        .bind(mission_id)
        .execute(&mut *transaction)
        .await?
        .rows_affected();

        if changed != 1 {
            return Err(StoreError::InvalidInput(
                "Mission state changed before approval could be recorded.".to_string(),
            ));
        }

        transaction.commit().await?;

        Ok(sqlx::query_as::<_, Mission>(
            "SELECT id, project_id, title, objective, status, created_at, updated_at
             FROM missions WHERE id = ?",
        )
        .bind(mission_id)
        .fetch_one(&self.pool)
        .await?)
    }
}

/// Project names are trimmed and compared using Unicode lowercase. This
/// intentionally does not apply Unicode NFC/NFD normalization, so visually
/// equivalent but differently normalized code-point sequences remain distinct.
fn normalize_project_name(name: &str) -> String {
    name.trim().to_lowercase()
}

async fn project_name_conflicts(
    transaction: &mut Transaction<'_, Sqlite>,
    name: &str,
    exclude_project_id: Option<&str>,
) -> Result<bool, StoreError> {
    let existing: Vec<(String, String)> = sqlx::query_as("SELECT id, name FROM projects")
        .fetch_all(&mut **transaction)
        .await?;
    let normalized = normalize_project_name(name);

    Ok(existing.into_iter().any(|(id, existing_name)| {
        exclude_project_id != Some(id.as_str())
            && normalize_project_name(&existing_name) == normalized
    }))
}

fn validate_text(field: &str, value: &str, max_chars: usize) -> Result<String, StoreError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(StoreError::InvalidInput(format!("{field} is required")));
    }
    if value.chars().count() > max_chars {
        return Err(StoreError::InvalidInput(format!(
            "{field} must be {max_chars} characters or fewer"
        )));
    }
    Ok(value.to_string())
}

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn collect_inventory(root: &Path) -> (Vec<InventoryFile>, bool, Vec<String>) {
    let mut files = Vec::new();
    let mut warnings = Vec::new();
    let mut truncated = false;

    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .follow_links(false)
        .git_ignore(true)
        .git_exclude(true)
        .parents(true)
        .max_depth(Some(MAX_WALK_DEPTH))
        .filter_entry(|entry| !is_excluded_name(&entry.file_name().to_string_lossy()));

    for (entries_seen, result) in builder.build().enumerate() {
        if entries_seen >= MAX_INVENTORY_ENTRIES {
            truncated = true;
            break;
        }

        let entry = match result {
            Ok(entry) => entry,
            Err(error) => {
                if warnings.len() < MAX_WARNINGS {
                    warnings.push(error.to_string());
                }
                continue;
            }
        };

        let Some(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_file() || file_type.is_symlink() {
            continue;
        }

        // symlink_metadata does not follow links; re-check after the walker
        // reports the entry to avoid trusting a stale file-type observation.
        let metadata = match fs::symlink_metadata(entry.path()) {
            Ok(metadata) if metadata.file_type().is_file() => metadata,
            Ok(_) => continue,
            Err(error) => {
                if warnings.len() < MAX_WARNINGS {
                    warnings.push(format!("Could not inspect an entry: {error}"));
                }
                continue;
            }
        };

        if files.len() >= MAX_INVENTORY_FILES {
            truncated = true;
            break;
        }

        let Ok(relative) = entry.path().strip_prefix(root) else {
            continue;
        };

        let extension = relative
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_else(|| "[no extension]".to_string());

        files.push(InventoryFile {
            path: relative.to_string_lossy().replace('\\', "/"),
            bytes: metadata.len(),
            extension,
        });
    }

    files.sort_by(|left, right| left.path.cmp(&right.path));
    (files, truncated, warnings)
}

fn is_excluded_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();

    matches!(
        lower.as_str(),
        ".git"
            | "node_modules"
            | "target"
            | "dist"
            | "build"
            | "vendor"
            | ".venv"
            | "venv"
            | "__pycache__"
            | ".rocksoul"
            | ".ssh"
            | ".aws"
            | ".azure"
            | ".kube"
            | ".gnupg"
            | ".config"
            | "secrets"
            | "credentials"
            | "private"
            | ".npmrc"
            | ".pypirc"
            | ".netrc"
            | "credentials.json"
            | "secrets.json"
            | "id_rsa"
            | "id_ecdsa"
            | "id_ed25519"
    ) || lower == ".env"
        || lower.starts_with(".env.")
        || lower.ends_with(".pem")
        || lower.ends_with(".key")
        || lower.ends_with(".p12")
        || lower.ends_with(".pfx")
        || lower.ends_with(".kdbx")
        || lower.ends_with(".p8")
        || lower.ends_with(".p7b")
        || lower.ends_with(".crt")
        || lower.ends_with(".cer")
        || lower.ends_with(".gpg")
        || lower.ends_with(".asc")
        || lower.ends_with(".age")
        || lower.ends_with(".secret")
        || lower.ends_with(".token")
        || lower.ends_with(".sqlite")
        || lower.ends_with(".sqlite3")
        || lower.ends_with(".db")
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use tempfile::tempdir;

    use super::Store;
    use super::collect_inventory;
    use super::{MAX_INVENTORY_ENTRIES, MAX_INVENTORY_FILES, MAX_WARNINGS};

    #[tokio::test]
    async fn project_mission_inventory_and_evidence_survive_reopen() -> Result<(), Box<dyn Error>> {
        let temporary = tempdir()?;
        let workspace = temporary.path().join("workspace");
        fs::create_dir_all(workspace.join("src"))?;
        fs::create_dir_all(workspace.join("target"))?;
        fs::write(workspace.join("README.md"), "# demo\n")?;
        fs::write(workspace.join(".gitignore"), "target/\n")?;
        fs::write(workspace.join("src/main.rs"), "fn main() {}\n")?;
        fs::write(workspace.join(".env"), "SECRET=never-list-this\n")?;
        fs::write(workspace.join("identity.pem"), "private material\n")?;
        fs::write(workspace.join("target/output.bin"), "ignored build output")?;

        let database = temporary.path().join("rocksoul.sqlite3");
        let store = Store::connect(&database).await?;
        let project = store
            .create_project("Rocksoul demo", workspace.to_str().unwrap_or_default())
            .await?;
        let mission = store
            .create_mission(
                &project.id,
                "Map this workspace",
                "Create a safe inventory of repository structure.",
            )
            .await?;

        assert!(store.approve_mission(&mission.id).await.is_err());
        let result = store.inspect_workspace(&project.id, &mission.id).await?;
        assert_eq!(result.file_count, 3);
        assert_eq!(result.artifact.sha256.len(), 64);
        assert!(result.artifact.content.contains("README.md"));
        assert!(result.artifact.content.contains("src/main.rs"));
        assert!(!result.artifact.content.contains(".env"));
        assert!(!result.artifact.content.contains("identity.pem"));
        assert!(!result.artifact.content.contains("target/output.bin"));
        assert!(
            result
                .evidence
                .details_json
                .contains("\"modifiedWorkspace\":false")
        );

        let missions = store.list_missions(&project.id).await?;
        assert_eq!(missions[0].status, "review");
        assert_eq!(store.list_artifacts(&mission.id).await?.len(), 1);
        assert_eq!(store.list_evidence(&mission.id).await?.len(), 1);
        assert!(
            store
                .inspect_workspace(&project.id, &mission.id)
                .await
                .is_err()
        );

        let approved = store.approve_mission(&mission.id).await?;
        assert_eq!(approved.status, "completed");
        assert!(store.approve_mission(&mission.id).await.is_err());
        let approval_evidence = store.list_evidence(&mission.id).await?;
        assert_eq!(approval_evidence.len(), 2);
        assert!(approval_evidence.iter().any(|item| {
            item.kind == "human_approval"
                && item.artifact_id.as_deref() == Some(result.artifact.id.as_str())
        }));

        drop(store);
        let reopened = Store::connect(&database).await?;
        assert_eq!(reopened.list_projects().await?.len(), 1);
        let reopened_missions = reopened.list_missions(&project.id).await?;
        assert_eq!(reopened_missions.len(), 1);
        assert_eq!(reopened_missions[0].status, "completed");
        assert_eq!(reopened.list_artifacts(&mission.id).await?.len(), 1);
        assert_eq!(reopened.list_evidence(&mission.id).await?.len(), 2);

        assert!(!workspace.join(".rocksoul").exists());
        assert_eq!(fs::read_to_string(workspace.join("README.md"))?, "# demo\n");
        Ok(())
    }

    #[tokio::test]
    async fn rejects_nonexistent_project_folder_without_writing_record()
    -> Result<(), Box<dyn Error>> {
        let temporary = tempdir()?;
        let store = Store::connect(&temporary.path().join("rocksoul.sqlite3")).await?;
        let missing = temporary.path().join("missing");

        assert!(
            store
                .create_project("No folder", missing.to_str().unwrap_or_default())
                .await
                .is_err()
        );
        assert!(store.list_projects().await?.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn mission_cannot_be_used_outside_its_project() -> Result<(), Box<dyn Error>> {
        let temporary = tempdir()?;
        let workspace = temporary.path().join("workspace");
        fs::create_dir_all(&workspace)?;
        let store = Store::connect(&temporary.path().join("rocksoul.sqlite3")).await?;
        let project = store
            .create_project("Rocksoul", workspace.to_str().unwrap_or_default())
            .await?;

        assert!(
            store
                .inspect_workspace(&project.id, "not-a-mission")
                .await
                .is_err()
        );
        Ok(())
    }

    #[test]
    fn inventory_caps_total_entries_even_when_folders_are_empty() -> Result<(), Box<dyn Error>> {
        let temporary = tempdir()?;
        for index in 0..(MAX_INVENTORY_ENTRIES + 50) {
            fs::create_dir(temporary.path().join(format!("empty-{index:04}")))?;
        }

        let (files, truncated, warnings) = collect_inventory(temporary.path());
        assert!(truncated);
        assert!(files.len() <= MAX_INVENTORY_FILES);
        assert!(warnings.len() <= MAX_WARNINGS);
        Ok(())
    }

    #[tokio::test]
    async fn validates_required_text_fields() -> Result<(), Box<dyn Error>> {
        let temporary = tempdir()?;
        let workspace = temporary.path().join("workspace");
        fs::create_dir_all(&workspace)?;
        let store = Store::connect(&temporary.path().join("rocksoul.sqlite3")).await?;
        assert!(
            store
                .create_project("  ", workspace.to_str().unwrap_or_default())
                .await
                .is_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn rename_preserves_project_identity_and_history_and_rejects_invalid_names()
    -> Result<(), Box<dyn Error>> {
        use crate::StoreError;

        let temporary = tempdir()?;
        let workspace = temporary.path().join("workspace");
        let other_workspace = temporary.path().join("other-workspace");
        fs::create_dir_all(&workspace)?;
        fs::create_dir_all(&other_workspace)?;
        fs::write(workspace.join("README.md"), "# rename fixture\n")?;

        let database = temporary.path().join("rocksoul.sqlite3");
        let store = Store::connect(&database).await?;
        let project = store
            .create_project("Original name", workspace.to_str().unwrap_or_default())
            .await?;
        let occupied = store
            .create_project(
                "Occupied name",
                other_workspace.to_str().unwrap_or_default(),
            )
            .await?;

        let mission = store
            .create_mission(
                &project.id,
                "Preserved mission",
                "Renaming a project must not change its mission history.",
            )
            .await?;
        let inspection = store.inspect_workspace(&project.id, &mission.id).await?;
        store.approve_mission(&mission.id).await?;

        let renamed = store
            .rename_project(&project.id, "  New project name  ")
            .await?;
        assert_eq!(renamed.id, project.id);
        assert_eq!(renamed.name, "New project name");
        assert_eq!(renamed.root_path, project.root_path);
        assert_eq!(renamed.created_at, project.created_at);

        assert!(matches!(
            store.rename_project(&project.id, " occupied NAME ").await,
            Err(StoreError::ProjectNameConflict)
        ));
        assert!(matches!(
            store.rename_project(&project.id, "   ").await,
            Err(StoreError::InvalidInput(_))
        ));
        assert!(matches!(
            store.rename_project(&project.id, &"x".repeat(81)).await,
            Err(StoreError::InvalidInput(_))
        ));
        assert!(matches!(
            store.rename_project("missing-project", "Valid name").await,
            Err(StoreError::NotFound)
        ));
        assert!(matches!(
            store.create_project(
                " new PROJECT name ",
                other_workspace.to_str().unwrap_or_default()
            )
                .await,
            Err(StoreError::ProjectNameConflict)
        ));

        let after_rejected_rename = store
            .list_projects()
            .await?
            .into_iter()
            .find(|item| item.id == project.id)
            .expect("renamed project remains present");
        assert_eq!(after_rejected_rename.name, "New project name");
        assert_eq!(after_rejected_rename.root_path, project.root_path);

        let missions = store.list_missions(&project.id).await?;
        assert_eq!(missions.len(), 1);
        assert_eq!(missions[0].id, mission.id);
        assert_eq!(missions[0].status, "completed");
        assert_eq!(store.list_artifacts(&mission.id).await?.len(), 1);
        assert_eq!(
            store.list_artifacts(&mission.id).await?[0].id,
            inspection.artifact.id
        );
        assert_eq!(store.list_evidence(&mission.id).await?.len(), 2);
        assert_eq!(
            fs::read_to_string(workspace.join("README.md"))?,
            "# rename fixture\n"
        );

        drop(store);
        let reopened = Store::connect(&database).await?;
        let restored = reopened
            .list_projects()
            .await?
            .into_iter()
            .find(|item| item.id == project.id)
            .expect("renamed project survives restart");
        assert_eq!(restored.name, "New project name");
        assert_eq!(restored.root_path, project.root_path);
        assert_eq!(reopened.list_missions(&project.id).await?.len(), 1);
        assert_eq!(reopened.list_artifacts(&mission.id).await?.len(), 1);
        assert_eq!(reopened.list_evidence(&mission.id).await?.len(), 2);
        assert_ne!(occupied.id, restored.id);
        Ok(())
    }
}
