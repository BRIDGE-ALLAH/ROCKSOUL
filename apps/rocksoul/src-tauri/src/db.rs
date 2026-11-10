use chrono::{SecondsFormat, Utc};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    FromRow, SqlitePool,
};
use std::{path::PathBuf, time::Duration};
use uuid::Uuid;

use crate::scanner::InspectionReport;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub workspace_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub brief: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub id: String,
    pub mission_id: String,
    pub title: String,
    pub content: String,
    pub sha256: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    pub id: String,
    pub project_id: String,
    pub mission_id: Option<String>,
    pub artifact_id: Option<String>,
    pub event_type: String,
    pub summary: String,
    pub details_json: String,
    pub created_at: String,
}

impl Database {
    pub async fn connect(path: PathBuf) -> Result<Self, String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("Cannot create local data folder: {error}"))?;
        }

        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_secs(5));

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|error| format!("Cannot open Rocksoul database: {error}"))?;

        Self::initialize(pool).await
    }

    async fn initialize(pool: SqlitePool) -> Result<Self, String> {
        let statements = [
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL CHECK(length(trim(name)) BETWEEN 1 AND 80),
                workspace_path TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS missions (
                id TEXT PRIMARY KEY NOT NULL,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                title TEXT NOT NULL CHECK(length(trim(title)) BETWEEN 1 AND 120),
                brief TEXT NOT NULL CHECK(length(brief) <= 12000),
                status TEXT NOT NULL CHECK(status IN ('ready', 'active', 'review', 'completed')),
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS artifacts (
                id TEXT PRIMARY KEY NOT NULL,
                mission_id TEXT NOT NULL REFERENCES missions(id) ON DELETE CASCADE,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                sha256 TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS evidence (
                id TEXT PRIMARY KEY NOT NULL,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                mission_id TEXT REFERENCES missions(id) ON DELETE CASCADE,
                artifact_id TEXT REFERENCES artifacts(id) ON DELETE SET NULL,
                event_type TEXT NOT NULL,
                summary TEXT NOT NULL,
                details_json TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY NOT NULL,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                mission_id TEXT REFERENCES missions(id) ON DELETE CASCADE,
                event_type TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            "CREATE INDEX IF NOT EXISTS idx_missions_project_updated ON missions(project_id, updated_at DESC)",
            "CREATE INDEX IF NOT EXISTS idx_artifacts_mission_created ON artifacts(mission_id, created_at DESC)",
            "CREATE INDEX IF NOT EXISTS idx_evidence_project_created ON evidence(project_id, created_at DESC)",
            "CREATE INDEX IF NOT EXISTS idx_events_project_created ON events(project_id, created_at DESC)",
            "PRAGMA user_version = 1",
        ];

        for statement in statements {
            sqlx::query(statement)
                .execute(&pool)
                .await
                .map_err(|error| format!("Cannot initialize local database: {error}"))?;
        }

        Ok(Self { pool })
    }

    #[cfg(test)]
    async fn memory() -> Self {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("in-memory SQLite");
        Self::initialize(pool).await.expect("initialize in-memory DB")
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>, String> {
        sqlx::query_as::<_, Project>(
            "SELECT id, name, workspace_path, created_at, updated_at
             FROM projects ORDER BY updated_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_error)
    }

    pub async fn create_project(&self, name: String) -> Result<Project, String> {
        let name = validate_text(&name, "Project name", 80)?;
        let id = Uuid::new_v4().to_string();
        let now = timestamp();
        let mut tx = self.pool.begin().await.map_err(db_error)?;

        sqlx::query(
            "INSERT INTO projects (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&name)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        sqlx::query(
            "INSERT INTO events (id, project_id, event_type, payload_json, created_at)
             VALUES (?, ?, 'project.created', ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&id)
        .bind(json!({"name": name}).to_string())
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        tx.commit().await.map_err(db_error)?;
        self.get_project(&id).await
    }

    pub async fn get_project(&self, id: &str) -> Result<Project, String> {
        sqlx::query_as::<_, Project>(
            "SELECT id, name, workspace_path, created_at, updated_at FROM projects WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_error)?
        .ok_or_else(|| "Project not found.".to_string())
    }

    pub async fn list_missions(&self, project_id: &str) -> Result<Vec<Mission>, String> {
        sqlx::query_as::<_, Mission>(
            "SELECT id, project_id, title, brief, status, created_at, updated_at
             FROM missions WHERE project_id = ? ORDER BY updated_at DESC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_error)
    }

    pub async fn create_mission(
        &self,
        project_id: String,
        title: String,
        brief: String,
    ) -> Result<Mission, String> {
        let title = validate_text(&title, "Mission title", 120)?;
        let brief = brief.trim().to_string();
        if brief.len() > 12_000 {
            return Err("Mission brief must be 12,000 bytes or less.".into());
        }
        if brief.is_empty() {
            return Err("Add a short brief before creating the mission.".into());
        }

        let project = self.get_project(&project_id).await?;
        let id = Uuid::new_v4().to_string();
        let now = timestamp();
        let mut tx = self.pool.begin().await.map_err(db_error)?;

        sqlx::query(
            "INSERT INTO missions (id, project_id, title, brief, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, 'ready', ?, ?)",
        )
        .bind(&id)
        .bind(&project_id)
        .bind(&title)
        .bind(&brief)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        sqlx::query(
            "INSERT INTO events (id, project_id, mission_id, event_type, payload_json, created_at)
             VALUES (?, ?, ?, 'mission.created', ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&project_id)
        .bind(&id)
        .bind(json!({"title": title, "status": "ready"}).to_string())
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        tx.commit().await.map_err(db_error)?;
        self.get_mission(&id).await
    }

    pub async fn get_mission(&self, id: &str) -> Result<Mission, String> {
        sqlx::query_as::<_, Mission>(
            "SELECT id, project_id, title, brief, status, created_at, updated_at
             FROM missions WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_error)?
        .ok_or_else(|| "Mission not found.".to_string())
    }

    pub async fn start_mission(&self, id: &str) -> Result<Mission, String> {
        let mission = self.get_mission(id).await?;
        if mission.status != "ready" {
            return Err("Only a ready mission can be started.".into());
        }

        let now = timestamp();
        let mut tx = self.pool.begin().await.map_err(db_error)?;
        sqlx::query("UPDATE missions SET status = 'active', updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;
        sqlx::query(
            "INSERT INTO events (id, project_id, mission_id, event_type, payload_json, created_at)
             VALUES (?, ?, ?, 'mission.started', '{}', ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&mission.project_id)
        .bind(id)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;
        tx.commit().await.map_err(db_error)?;

        self.get_mission(id).await
    }

    pub async fn save_artifact(
        &self,
        mission_id: &str,
        title: String,
        content: String,
    ) -> Result<Artifact, String> {
        let title = validate_text(&title, "Artifact title", 120)?;
        if content.trim().is_empty() {
            return Err("Artifact content cannot be empty.".into());
        }
        if content.len() > 1_000_000 {
            return Err("Artifact content must be 1 MB or less.".into());
        }

        let mission = self.get_mission(mission_id).await?;
        if mission.status != "active" {
            return Err("Start the mission before saving an artifact.".into());
        }

        let id = Uuid::new_v4().to_string();
        let now = timestamp();
        let sha256 = format!("{:x}", Sha256::digest(content.as_bytes()));
        let details = json!({"sha256": sha256, "title": title, "bytes": content.len()});
        let mut tx = self.pool.begin().await.map_err(db_error)?;

        sqlx::query(
            "INSERT INTO artifacts (id, mission_id, title, content, sha256, created_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(mission_id)
        .bind(&title)
        .bind(&content)
        .bind(&sha256)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        sqlx::query(
            "INSERT INTO evidence
             (id, project_id, mission_id, artifact_id, event_type, summary, details_json, created_at)
             VALUES (?, ?, ?, ?, 'artifact.saved', ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&mission.project_id)
        .bind(mission_id)
        .bind(&id)
        .bind(format!("Artifact saved with SHA-256 {sha256}."))
        .bind(details.to_string())
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        sqlx::query("UPDATE missions SET status = 'review', updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(mission_id)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;

        sqlx::query(
            "INSERT INTO events (id, project_id, mission_id, event_type, payload_json, created_at)
             VALUES (?, ?, ?, 'artifact.saved', ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&mission.project_id)
        .bind(mission_id)
        .bind(json!({"artifact_id": id, "sha256": sha256}).to_string())
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        tx.commit().await.map_err(db_error)?;
        self.get_artifact(&id).await
    }

    pub async fn get_artifact(&self, id: &str) -> Result<Artifact, String> {
        sqlx::query_as::<_, Artifact>(
            "SELECT id, mission_id, title, content, sha256, created_at FROM artifacts WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_error)?
        .ok_or_else(|| "Artifact not found.".to_string())
    }

    pub async fn list_artifacts(&self, mission_id: &str) -> Result<Vec<Artifact>, String> {
        sqlx::query_as::<_, Artifact>(
            "SELECT id, mission_id, title, content, sha256, created_at
             FROM artifacts WHERE mission_id = ? ORDER BY created_at DESC",
        )
        .bind(mission_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_error)
    }

    pub async fn complete_mission(&self, id: &str) -> Result<Mission, String> {
        let mission = self.get_mission(id).await?;
        if mission.status != "review" {
            return Err("A mission needs a saved artifact and evidence before completion.".into());
        }

        let evidence_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM evidence WHERE mission_id = ?")
                .bind(id)
                .fetch_one(&self.pool)
                .await
                .map_err(db_error)?;
        if evidence_count < 1 {
            return Err("Completion blocked: no evidence is recorded.".into());
        }

        let now = timestamp();
        let mut tx = self.pool.begin().await.map_err(db_error)?;
        sqlx::query("UPDATE missions SET status = 'completed', updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;
        sqlx::query(
            "INSERT INTO events (id, project_id, mission_id, event_type, payload_json, created_at)
             VALUES (?, ?, ?, 'mission.completed', ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&mission.project_id)
        .bind(id)
        .bind(json!({"evidence_count": evidence_count}).to_string())
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;
        tx.commit().await.map_err(db_error)?;

        self.get_mission(id).await
    }

    pub async fn list_evidence(&self, project_id: &str) -> Result<Vec<Evidence>, String> {
        sqlx::query_as::<_, Evidence>(
            "SELECT id, project_id, mission_id, artifact_id, event_type, summary, details_json, created_at
             FROM evidence WHERE project_id = ? ORDER BY created_at DESC LIMIT 100",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_error)
    }

    pub async fn save_inspection(
        &self,
        project_id: &str,
        report: &InspectionReport,
    ) -> Result<(), String> {
        let _project = self.get_project(project_id).await?;
        let now = timestamp();
        let details = serde_json::to_string(report).map_err(|error| error.to_string())?;
        let summary = format!(
            "Read-only inventory: {} files in {} directories; {} sensitive names and {} symlinks skipped{}.",
            report.files_scanned,
            report.directories_scanned,
            report.skipped_sensitive,
            report.skipped_symlinks,
            if report.truncated { " (scan limit reached)" } else { "" }
        );
        let mut tx = self.pool.begin().await.map_err(db_error)?;

        sqlx::query("UPDATE projects SET workspace_path = ?, updated_at = ? WHERE id = ?")
            .bind(&report.root_path)
            .bind(&now)
            .bind(project_id)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;

        sqlx::query(
            "INSERT INTO evidence
             (id, project_id, event_type, summary, details_json, created_at)
             VALUES (?, ?, 'workspace.inspected', ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(project_id)
        .bind(&summary)
        .bind(details)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        sqlx::query(
            "INSERT INTO events (id, project_id, event_type, payload_json, created_at)
             VALUES (?, ?, 'workspace.inspected', ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(project_id)
        .bind(json!({
            "files_scanned": report.files_scanned,
            "directories_scanned": report.directories_scanned,
            "truncated": report.truncated
        }).to_string())
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        tx.commit().await.map_err(db_error)
    }
}

fn validate_text(value: &str, label: &str, max_bytes: usize) -> Result<String, String> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(format!("{label} is required."));
    }
    if normalized.len() > max_bytes {
        return Err(format!("{label} must be {max_bytes} bytes or less."));
    }
    if normalized.chars().any(char::is_control) {
        return Err(format!("{label} contains unsupported control characters."));
    }
    Ok(normalized.to_string())
}

fn timestamp() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn db_error(error: sqlx::Error) -> String {
    format!("Local database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mission_cannot_finish_without_artifact_and_evidence() {
        let db = Database::memory().await;
        let project = db.create_project("Chaos Lab".into()).await.expect("project");
        let mission = db
            .create_mission(project.id, "Build a prototype".into(), "Make a first working take.".into())
            .await
            .expect("mission");

        assert_eq!(mission.status, "ready");
        assert!(db.complete_mission(&mission.id).await.is_err());
        assert!(db.save_artifact(&mission.id, "Take one".into(), "not yet active".into()).await.is_err());

        let active = db.start_mission(&mission.id).await.expect("start");
        assert_eq!(active.status, "active");
        let artifact = db
            .save_artifact(&mission.id, "First take".into(), "# Prototype\n\nA real artifact.".into())
            .await
            .expect("artifact");
        assert_eq!(artifact.sha256.len(), 64);
        assert_eq!(db.get_mission(&mission.id).await.expect("mission").status, "review");

        let completed = db.complete_mission(&mission.id).await.expect("complete");
        assert_eq!(completed.status, "completed");
        let evidence = db.list_evidence(&completed.project_id).await.expect("evidence");
        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].event_type, "artifact.saved");
    }

    #[tokio::test]
    async fn database_reopens_with_project_and_mission_state() {
        let folder = tempfile::tempdir().expect("tempdir");
        let path = folder.path().join("rocksoul.sqlite");
        let project_id;

        {
            let db = Database::connect(path.clone()).await.expect("open");
            let project = db.create_project("Persistent Studio".into()).await.expect("project");
            project_id = project.id.clone();
            db.create_mission(project.id, "Keep the thread".into(), "Persist this mission.".into())
                .await
                .expect("mission");
        }

        let reopened = Database::connect(path).await.expect("reopen");
        let projects = reopened.list_projects().await.expect("list");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, project_id);
        let missions = reopened.list_missions(&project_id).await.expect("missions");
        assert_eq!(missions.len(), 1);
        assert_eq!(missions[0].status, "ready");
    }

    #[tokio::test]
    async fn rejects_invalid_names_and_empty_briefs() {
        let db = Database::memory().await;
        assert!(db.create_project("   ".into()).await.is_err());
        let project = db.create_project("Studio".into()).await.expect("project");
        assert!(db.create_mission(project.id, "Mission".into(), " ".into()).await.is_err());
    }

    #[tokio::test]
    async fn mission_state_transitions_are_not_skippable() {
        let db = Database::memory().await;
        let project = db.create_project("Studio".into()).await.expect("project");
        let mission = db.create_mission(project.id, "Mission".into(), "Brief".into()).await.expect("mission");
        assert!(db.start_mission(&mission.id).await.is_ok());
        assert!(db.start_mission(&mission.id).await.is_err());
        assert!(db.complete_mission(&mission.id).await.is_err());
    }
}
