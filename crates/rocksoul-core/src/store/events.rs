use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Sqlite, Transaction};
use uuid::Uuid;

use super::Store;
use crate::StoreError;

pub const CURRENT_EVENT_SCHEMA_VERSION: i64 = 1;
pub const MAX_EVENT_PAYLOAD_BYTES: usize = 16 * 1024;
const MAX_EVENT_STRING_BYTES: usize = 2 * 1024;
const MAX_EVENT_OBJECT_FIELDS: usize = 64;
const MAX_EVENT_ARRAY_ITEMS: usize = 100;
const MAX_EVENT_DEPTH: usize = 8;
const MAX_EVENT_PAGE_SIZE: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    ProjectCreated,
    ProjectRenamed,
    ProjectArchived,
    ProjectRestored,
    MissionCreated,
    MissionDraftUpdated,
    MissionInspected,
    MissionApproved,
    BackupCreated,
}

impl EventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProjectCreated => "project_created",
            Self::ProjectRenamed => "project_renamed",
            Self::ProjectArchived => "project_archived",
            Self::ProjectRestored => "project_restored",
            Self::MissionCreated => "mission_created",
            Self::MissionDraftUpdated => "mission_draft_updated",
            Self::MissionInspected => "mission_inspected",
            Self::MissionApproved => "mission_approved",
            Self::BackupCreated => "backup_created",
        }
    }
}

impl FromStr for EventType {
    type Err = StoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "project_created" => Ok(Self::ProjectCreated),
            "project_renamed" => Ok(Self::ProjectRenamed),
            "project_archived" => Ok(Self::ProjectArchived),
            "project_restored" => Ok(Self::ProjectRestored),
            "mission_created" => Ok(Self::MissionCreated),
            "mission_draft_updated" => Ok(Self::MissionDraftUpdated),
            "mission_inspected" => Ok(Self::MissionInspected),
            "mission_approved" => Ok(Self::MissionApproved),
            "backup_created" => Ok(Self::BackupCreated),
            other => Err(StoreError::InvalidEventRecord(format!(
                "unknown event type {other:?}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventSubjectType {
    Project,
    Mission,
    Backup,
}

impl EventSubjectType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Mission => "mission",
            Self::Backup => "backup",
        }
    }
}

impl FromStr for EventSubjectType {
    type Err = StoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "project" => Ok(Self::Project),
            "mission" => Ok(Self::Mission),
            "backup" => Ok(Self::Backup),
            other => Err(StoreError::InvalidEventRecord(format!(
                "unknown subject type {other:?}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventActorType {
    User,
    System,
}

impl EventActorType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::System => "system",
        }
    }
}

impl FromStr for EventActorType {
    type Err = StoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "user" => Ok(Self::User),
            "system" => Ok(Self::System),
            other => Err(StoreError::InvalidEventRecord(format!(
                "unknown actor type {other:?}"
            ))),
        }
    }
}

/// Caller-supplied event data. The Store owns the ID and event timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDraft {
    pub event_type: EventType,
    pub schema_version: i64,
    pub subject_type: EventSubjectType,
    pub subject_id: String,
    pub actor_type: Option<EventActorType>,
    pub actor_id: Option<String>,
    pub correlation_id: Option<String>,
    pub payload: Value,
}

impl EventDraft {
    pub fn new(
        event_type: EventType,
        subject_type: EventSubjectType,
        subject_id: impl Into<String>,
        payload: Value,
    ) -> Self {
        Self {
            event_type,
            schema_version: CURRENT_EVENT_SCHEMA_VERSION,
            subject_type,
            subject_id: subject_id.into(),
            actor_type: None,
            actor_id: None,
            correlation_id: None,
            payload,
        }
    }

    pub fn with_actor(mut self, actor_type: EventActorType, actor_id: impl Into<String>) -> Self {
        self.actor_type = Some(actor_type);
        self.actor_id = Some(actor_id.into());
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRecord {
    pub id: String,
    pub event_type: EventType,
    pub schema_version: i64,
    pub subject_type: EventSubjectType,
    pub subject_id: String,
    pub actor_type: Option<EventActorType>,
    pub actor_id: Option<String>,
    pub occurred_at: String,
    pub correlation_id: Option<String>,
    pub payload: Value,
}

#[derive(Debug, FromRow)]
struct EventRow {
    id: String,
    event_type: String,
    schema_version: i64,
    subject_type: String,
    subject_id: String,
    actor_type: Option<String>,
    actor_id: Option<String>,
    occurred_at: String,
    correlation_id: Option<String>,
    payload_json: String,
}

impl TryFrom<EventRow> for EventRecord {
    type Error = StoreError;

    fn try_from(row: EventRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            event_type: row.event_type.parse()?,
            schema_version: row.schema_version,
            subject_type: row.subject_type.parse()?,
            subject_id: row.subject_id,
            actor_type: row.actor_type.map(|value| value.parse()).transpose()?,
            actor_id: row.actor_id,
            occurred_at: row.occurred_at,
            correlation_id: row.correlation_id,
            payload: serde_json::from_str(&row.payload_json)
                .map_err(|error| StoreError::InvalidEventRecord(error.to_string()))?,
        })
    }
}

fn validate_identifier(field: &str, value: &str) -> Result<(), StoreError> {
    if value.trim().is_empty()
        || value.trim() != value
        || value.len() > 256
        || value.chars().any(char::is_control)
    {
        return Err(StoreError::InvalidEventPayload(format!(
            "{field} must be trimmed, non-empty, and at most 256 bytes"
        )));
    }
    Ok(())
}

fn validate_payload_node(value: &Value, depth: usize) -> Result<(), StoreError> {
    if depth > MAX_EVENT_DEPTH {
        return Err(StoreError::InvalidEventPayload(format!(
            "payload nesting exceeds {MAX_EVENT_DEPTH} levels"
        )));
    }

    match value {
        Value::Object(object) => {
            if object.len() > MAX_EVENT_OBJECT_FIELDS {
                return Err(StoreError::InvalidEventPayload(format!(
                    "an event object may contain at most {MAX_EVENT_OBJECT_FIELDS} fields"
                )));
            }
            for (key, child) in object {
                let normalized: String = key
                    .chars()
                    .filter(|character| character.is_ascii_alphanumeric())
                    .flat_map(char::to_lowercase)
                    .collect();
                let sensitive = [
                    "password",
                    "secret",
                    "token",
                    "authorization",
                    "apikey",
                    "privatekey",
                    "credential",
                    "prompt",
                    "sourcetext",
                    "documenttext",
                    "rawcontent",
                    "rawtext",
                    "filecontent",
                    "filecontents",
                    "documentbody",
                ];
                if sensitive.iter().any(|needle| normalized.contains(needle)) {
                    return Err(StoreError::InvalidEventPayload(format!(
                        "payload field {key:?} is not allowed; event payloads are metadata-only"
                    )));
                }
                validate_payload_node(child, depth + 1)?;
            }
        }
        Value::Array(items) => {
            if items.len() > MAX_EVENT_ARRAY_ITEMS {
                return Err(StoreError::InvalidEventPayload(format!(
                    "an event array may contain at most {MAX_EVENT_ARRAY_ITEMS} items"
                )));
            }
            for child in items {
                validate_payload_node(child, depth + 1)?;
            }
        }
        Value::String(text) if text.len() > MAX_EVENT_STRING_BYTES => {
            return Err(StoreError::InvalidEventPayload(format!(
                "a string value may contain at most {MAX_EVENT_STRING_BYTES} bytes"
            )));
        }
        _ => {}
    }
    Ok(())
}

fn validate_event_payload(event_type: EventType, payload: &Value) -> Result<(), StoreError> {
    let object = payload.as_object().ok_or_else(|| {
        StoreError::InvalidEventPayload(
            "event payload must be a JSON object containing metadata only".into(),
        )
    })?;

    for (field, value) in object {
        let valid = match (event_type, field.as_str()) {
            (EventType::ProjectCreated, "name_length")
            | (EventType::ProjectRenamed, "old_name_length")
            | (EventType::ProjectRenamed, "new_name_length")
            | (EventType::MissionCreated, "title_length")
            | (EventType::MissionCreated, "objective_length")
            | (EventType::MissionDraftUpdated, "title_length")
            | (EventType::MissionDraftUpdated, "objective_length")
            | (EventType::MissionInspected, "file_count")
            | (EventType::MissionInspected, "warning_count")
            | (EventType::BackupCreated, "backup_size_bytes")
            | (EventType::BackupCreated, "schema_version") => value.as_u64().is_some(),
            (EventType::ProjectArchived, "reason_code")
            | (EventType::ProjectRestored, "reason_code") => value.as_str().is_some_and(|text| {
                !text.is_empty()
                    && text.len() <= 32
                    && text.chars().all(|character| {
                        character.is_ascii_lowercase()
                            || character.is_ascii_digit()
                            || matches!(character, '.' | '_' | '-')
                    })
            }),
            (EventType::MissionInspected, "truncated")
            | (EventType::MissionApproved, "approved") => value.is_boolean(),
            (EventType::MissionInspected, "artifact_sha256")
            | (EventType::BackupCreated, "backup_sha256") => value.as_str().is_some_and(|hash| {
                hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_hexdigit())
            }),
            _ => false,
        };

        if !valid {
            return Err(StoreError::InvalidEventPayload(format!(
                "field {field:?} is not an approved metadata field for event type {}",
                event_type.as_str()
            )));
        }
    }
    Ok(())
}

fn validate_draft(draft: &EventDraft) -> Result<String, StoreError> {
    if draft.schema_version != CURRENT_EVENT_SCHEMA_VERSION {
        return Err(StoreError::UnsupportedEventSchema(draft.schema_version));
    }
    validate_identifier("subject_id", &draft.subject_id)?;
    match (&draft.actor_type, &draft.actor_id) {
        (Some(_), Some(actor_id)) => validate_identifier("actor_id", actor_id)?,
        (None, None) => {}
        _ => {
            return Err(StoreError::InvalidEventPayload(
                "actor_type and actor_id must either both be present or both be absent".into(),
            ));
        }
    }
    if let Some(correlation_id) = &draft.correlation_id {
        validate_identifier("correlation_id", correlation_id)?;
    }
    if !draft.payload.is_object() {
        return Err(StoreError::InvalidEventPayload(
            "event payload must be a JSON object containing metadata only".into(),
        ));
    }
    validate_event_payload(draft.event_type, &draft.payload)?;
    validate_payload_node(&draft.payload, 0)?;
    let serialized = serde_json::to_string(&draft.payload)
        .map_err(|error| StoreError::InvalidEventPayload(error.to_string()))?;
    if serialized.len() > MAX_EVENT_PAYLOAD_BYTES {
        return Err(StoreError::InvalidEventPayload(format!(
            "serialized payload exceeds {MAX_EVENT_PAYLOAD_BYTES} bytes"
        )));
    }
    Ok(serialized)
}

async fn insert_event(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    occurred_at: &str,
    draft: &EventDraft,
    payload_json: &str,
) -> Result<(), StoreError> {
    let actor_type = draft.actor_type.map(EventActorType::as_str);
    sqlx::query(
        "INSERT INTO event_ledger (
            id, event_type, schema_version, subject_type, subject_id,
            actor_type, actor_id, occurred_at, correlation_id, payload_json, payload_bytes
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(draft.event_type.as_str())
    .bind(draft.schema_version)
    .bind(draft.subject_type.as_str())
    .bind(&draft.subject_id)
    .bind(actor_type)
    .bind(&draft.actor_id)
    .bind(occurred_at)
    .bind(&draft.correlation_id)
    .bind(payload_json)
    .bind(payload_json.len() as i64)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

impl Store {
    /// Append an immutable, typed event. The transaction commits only after
    /// the row has been inserted successfully.
    pub async fn append_event(&self, draft: EventDraft) -> Result<EventRecord, StoreError> {
        let payload_json = validate_draft(&draft)?;
        let id = Uuid::new_v4().to_string();
        let occurred_at = chrono::Utc::now().to_rfc3339();

        let mut transaction = self.pool.begin().await?;
        insert_event(&mut transaction, &id, &occurred_at, &draft, &payload_json).await?;
        let row = sqlx::query_as::<_, EventRow>(
            "SELECT id, event_type, schema_version, subject_type, subject_id,
                    actor_type, actor_id, occurred_at, correlation_id, payload_json
             FROM event_ledger WHERE id = ?",
        )
        .bind(&id)
        .fetch_one(&mut *transaction)
        .await?;
        transaction.commit().await?;
        EventRecord::try_from(row)
    }

    /// List events for one project/mission/backup subject. The query has a
    /// hard upper bound and returns newest entries first.
    pub async fn list_subject_events(
        &self,
        subject_type: EventSubjectType,
        subject_id: &str,
        limit: u32,
    ) -> Result<Vec<EventRecord>, StoreError> {
        validate_identifier("subject_id", subject_id)?;
        if !(1..=MAX_EVENT_PAGE_SIZE).contains(&limit) {
            return Err(StoreError::InvalidInput(format!(
                "event page size must be between 1 and {MAX_EVENT_PAGE_SIZE}"
            )));
        }

        let rows = sqlx::query_as::<_, EventRow>(
            "SELECT id, event_type, schema_version, subject_type, subject_id,
                    actor_type, actor_id, occurred_at, correlation_id, payload_json
             FROM event_ledger
             WHERE subject_type = ? AND subject_id = ?
             ORDER BY occurred_at DESC, id DESC
             LIMIT ?",
        )
        .bind(subject_type.as_str())
        .bind(subject_id)
        .bind(i64::from(limit))
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(EventRecord::try_from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    fn event(subject_id: &str) -> EventDraft {
        EventDraft::new(
            EventType::MissionCreated,
            EventSubjectType::Mission,
            subject_id,
            json!({"title_length": 18, "objective_length": 42}),
        )
        .with_actor(EventActorType::User, "local-user")
        .with_correlation_id("mission-run-1")
    }

    #[tokio::test]
    async fn event_round_trip_is_typed_bounded_and_append_only()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempdir()?;
        let database = directory.path().join("rocksoul.sqlite3");
        let store = Store::connect(&database).await?;

        let written = store.append_event(event("mission-1")).await?;
        assert_eq!(written.event_type, EventType::MissionCreated);
        assert_eq!(written.subject_type, EventSubjectType::Mission);
        assert_eq!(written.subject_id, "mission-1");
        assert_eq!(written.actor_type, Some(EventActorType::User));
        assert_eq!(written.actor_id.as_deref(), Some("local-user"));
        assert_eq!(written.correlation_id.as_deref(), Some("mission-run-1"));
        assert_eq!(written.payload["title_length"], 18);
        assert_eq!(written.payload["objective_length"], 42);

        let listed = store
            .list_subject_events(EventSubjectType::Mission, "mission-1", 20)
            .await?;
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, written.id);
        assert_eq!(listed[0].payload, written.payload);

        let update = sqlx::query("UPDATE event_ledger SET payload_json = '{}' WHERE id = ?")
            .bind(&written.id)
            .execute(&store.pool)
            .await;
        assert!(
            update.is_err(),
            "event UPDATE must be rejected by the database trigger"
        );

        let delete = sqlx::query("DELETE FROM event_ledger WHERE id = ?")
            .bind(&written.id)
            .execute(&store.pool)
            .await;
        assert!(
            delete.is_err(),
            "event DELETE must be rejected by the database trigger"
        );

        Ok(())
    }

    #[tokio::test]
    async fn unsupported_schema_sensitive_fields_and_oversized_payloads_are_rejected()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempdir()?;
        let store = Store::connect(&directory.path().join("rocksoul.sqlite3")).await?;

        let mut unknown_schema = event("mission-1");
        unknown_schema.schema_version = CURRENT_EVENT_SCHEMA_VERSION + 1;
        assert!(matches!(
            store.append_event(unknown_schema).await,
            Err(StoreError::UnsupportedEventSchema(_))
        ));

        let mut sensitive = event("mission-1");
        sensitive.payload = json!({"access_token":"must-not-persist"});
        assert!(matches!(
            store.append_event(sensitive).await,
            Err(StoreError::InvalidEventPayload(_))
        ));

        let mut raw_content = event("mission-1");
        raw_content.payload = json!({"source_text":"private source contents"});
        assert!(matches!(
            store.append_event(raw_content).await,
            Err(StoreError::InvalidEventPayload(_))
        ));

        let mut unapproved_metadata = event("mission-1");
        unapproved_metadata.payload = json!({"source_count": 12});
        assert!(matches!(
            store.append_event(unapproved_metadata).await,
            Err(StoreError::InvalidEventPayload(_))
        ));

        let mut text_in_numeric_field = event("mission-1");
        text_in_numeric_field.payload = json!({"title_length":"private text"});
        assert!(matches!(
            store.append_event(text_in_numeric_field).await,
            Err(StoreError::InvalidEventPayload(_))
        ));

        let mut oversized = event("mission-1");
        oversized.payload = json!({"metadata":"x".repeat(MAX_EVENT_PAYLOAD_BYTES)});
        assert!(matches!(
            store.append_event(oversized).await,
            Err(StoreError::InvalidEventPayload(_))
        ));

        let mut array_payload = event("mission-1");
        array_payload.payload = json!([{"ok":true}]);
        assert!(matches!(
            store.append_event(array_payload).await,
            Err(StoreError::InvalidEventPayload(_))
        ));

        let invalid_actor = EventDraft {
            actor_type: None,
            actor_id: Some("missing-type".into()),
            ..event("mission-1")
        };
        assert!(matches!(
            store.append_event(invalid_actor).await,
            Err(StoreError::InvalidEventPayload(_))
        ));
        assert!(
            store
                .list_subject_events(EventSubjectType::Mission, "mission-1", 20)
                .await?
                .is_empty()
        );
        Ok(())
    }

    #[tokio::test]
    async fn event_ordering_is_newest_first_and_page_size_is_bounded()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempdir()?;
        let store = Store::connect(&directory.path().join("rocksoul.sqlite3")).await?;

        let first = store.append_event(event("mission-1")).await?;
        let second = store
            .append_event(EventDraft::new(
                EventType::MissionApproved,
                EventSubjectType::Mission,
                "mission-1",
                json!({"approved": true}),
            ))
            .await?;

        let events = store
            .list_subject_events(EventSubjectType::Mission, "mission-1", 10)
            .await?;
        assert_eq!(events.len(), 2);
        assert!(events.iter().any(|item| item.id == first.id));
        assert!(events.iter().any(|item| item.id == second.id));
        for pair in events.windows(2) {
            assert!(pair[0].occurred_at >= pair[1].occurred_at);
        }
        assert!(
            store
                .list_subject_events(EventSubjectType::Mission, "mission-1", 0)
                .await
                .is_err()
        );
        assert!(
            store
                .list_subject_events(
                    EventSubjectType::Mission,
                    "mission-1",
                    MAX_EVENT_PAGE_SIZE + 1
                )
                .await
                .is_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn event_insert_participates_in_a_transaction_and_rolls_back()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempdir()?;
        let store = Store::connect(&directory.path().join("rocksoul.sqlite3")).await?;
        let draft = event("mission-rollback");
        let payload_json = validate_draft(&draft)?;
        let id = Uuid::new_v4().to_string();
        let occurred_at = chrono::Utc::now().to_rfc3339();

        let mut transaction = store.pool.begin().await?;
        insert_event(&mut transaction, &id, &occurred_at, &draft, &payload_json).await?;
        transaction.rollback().await?;

        let rows = store
            .list_subject_events(EventSubjectType::Mission, "mission-rollback", 10)
            .await?;
        assert!(rows.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn reopening_an_existing_database_preserves_data_and_event_schema()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempdir()?;
        let database = directory.path().join("rocksoul.sqlite3");
        let first_store = Store::connect(&database).await?;
        let event = first_store.append_event(event("mission-upgrade")).await?;
        drop(first_store);

        let reopened = Store::connect(&database).await?;
        let records = reopened
            .list_subject_events(EventSubjectType::Mission, "mission-upgrade", 10)
            .await?;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, event.id);
        assert_eq!(records[0].payload, event.payload);

        let schema_version: i64 = sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
            .fetch_one(&reopened.pool)
            .await?;
        assert!(schema_version >= 4);
        Ok(())
    }
}
