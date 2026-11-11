use std::{path::Path, sync::Mutex};

use rocksoul_core::{Artifact, Evidence, InspectionResult, Mission, Project, Store};
use serde::Serialize;
use tauri::{Manager, State};
use tracing_appender::non_blocking::WorkerGuard;

mod lifecycle;

use lifecycle::{LifecycleSession, safe_store_error_code};

struct AppState {
    store: Option<Store>,
    startup_status: StartupStatus,
    lifecycle_session: Mutex<Option<LifecycleSession>>,
    trace_guard: Mutex<Option<WorkerGuard>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct StartupStatus {
    phase: String,
    recovery_warning: Option<String>,
    error_code: Option<String>,
}

impl StartupStatus {
    fn ready(recovery_warning: Option<String>) -> Self {
        Self {
            phase: "ready".to_string(),
            recovery_warning,
            error_code: None,
        }
    }

    fn failed(error_code: &'static str, recovery_warning: Option<String>) -> Self {
        Self {
            phase: "failed".to_string(),
            recovery_warning,
            error_code: Some(error_code.to_string()),
        }
    }
}

fn unavailable_store_error() -> String {
    "The local vault did not become ready. Restart Rocksoul after resolving the startup issue."
        .to_string()
}

fn startup_status_for_result(
    result: Result<Store, rocksoul_core::StoreError>,
    recovery_warning: Option<String>,
) -> (Option<Store>, StartupStatus) {
    match result {
        Ok(store) => (Some(store), StartupStatus::ready(recovery_warning)),
        Err(error) => (
            None,
            StartupStatus::failed(safe_store_error_code(&error), recovery_warning),
        ),
    }
}

#[tauri::command]
fn get_startup_status(state: State<'_, AppState>) -> StartupStatus {
    state.startup_status.clone()
}

#[tauri::command]
async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let store = state.store.as_ref().ok_or_else(unavailable_store_error)?;
    store.list_projects().await.map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn create_project(
    name: String,
    root_path: String,
    state: State<'_, AppState>,
) -> Result<Project, String> {
    let store = state.store.as_ref().ok_or_else(unavailable_store_error)?;
    store
        .create_project(&name, &root_path)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn list_missions(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Mission>, String> {
    let store = state.store.as_ref().ok_or_else(unavailable_store_error)?;
    store
        .list_missions(&project_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn create_mission(
    project_id: String,
    title: String,
    objective: String,
    state: State<'_, AppState>,
) -> Result<Mission, String> {
    let store = state.store.as_ref().ok_or_else(unavailable_store_error)?;
    store
        .create_mission(&project_id, &title, &objective)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn inspect_workspace(
    project_id: String,
    mission_id: String,
    state: State<'_, AppState>,
) -> Result<InspectionResult, String> {
    let store = state.store.as_ref().ok_or_else(unavailable_store_error)?;
    store
        .inspect_workspace(&project_id, &mission_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn list_artifacts(
    mission_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Artifact>, String> {
    let store = state.store.as_ref().ok_or_else(unavailable_store_error)?;
    store
        .list_artifacts(&mission_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn list_evidence(
    mission_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Evidence>, String> {
    let store = state.store.as_ref().ok_or_else(unavailable_store_error)?;
    store
        .list_evidence(&mission_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn approve_mission(
    mission_id: String,
    state: State<'_, AppState>,
) -> Result<Mission, String> {
    let store = state.store.as_ref().ok_or_else(unavailable_store_error)?;
    store
        .approve_mission(&mission_id)
        .await
        .map_err(|error| error.to_string())
}

fn initialize_tracing(log_directory: Option<&Path>) -> Option<WorkerGuard> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    if let Some(log_directory) = log_directory {
        if std::fs::create_dir_all(log_directory).is_ok() {
            let appender = tracing_appender::rolling::daily(log_directory, "rocksoul.jsonl");
            let (writer, guard) = tracing_appender::non_blocking(appender);
            let initialized = tracing_subscriber::fmt()
                .json()
                .with_env_filter(filter.clone())
                .with_writer(writer)
                .with_target(false)
                .with_current_span(false)
                .with_span_list(false)
                .try_init()
                .is_ok();
            if initialized {
                return Some(guard);
            }
            drop(guard);
        }
    }

    // stderr is a best-effort fallback for development or data-directory errors.
    let _ = tracing_subscriber::fmt()
        .json()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_current_span(false)
        .with_span_list(false)
        .try_init();
    None
}

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_dir_result = app.path().app_local_data_dir();
            let trace_guard = match &data_dir_result {
                Ok(data_dir) => initialize_tracing(Some(&data_dir.join("logs"))),
                Err(_) => initialize_tracing(None),
            };
            tracing::info!(
                event = "app_startup",
                stage = "runtime",
                outcome = "started"
            );

            let mut store = None;
            let mut startup_status = StartupStatus::failed(
                "application_data_directory_unavailable",
                None,
            );
            let mut lifecycle_session = None;

            match data_dir_result {
                Ok(data_dir) => {
                    let (session, recovery_warning) = match LifecycleSession::begin(&data_dir) {
                        Ok(started) => {
                            tracing::info!(
                                event = "app_startup",
                                stage = "session_marker",
                                outcome = "succeeded",
                                previous_unclean_exit = started.previous_unclean_exit
                            );
                            let warning = started.previous_unclean_exit.then(|| {
                                "A previous session ended before its clean-shutdown marker was cleared. The vault opened successfully; review the last operation before continuing.".to_string()
                            });
                            (Some(started.session), warning)
                        }
                        Err(_) => {
                            tracing::warn!(
                                event = "app_startup",
                                stage = "session_marker",
                                outcome = "failed",
                                error_code = "lifecycle_marker_unavailable"
                            );
                            (
                                None,
                                Some("Rocksoul could not verify the previous shutdown state. The database can still open, but crash-detection evidence is unavailable.".to_string()),
                            )
                        }
                    };
                    lifecycle_session = session;

                    let database_path = data_dir.join("rocksoul.sqlite3");
                    tracing::info!(
                        event = "app_startup",
                        stage = "database_open_and_migrate",
                        outcome = "started"
                    );
                    let result =
                        tauri::async_runtime::block_on(Store::connect(&database_path));
                    let (connected_store, status) =
                        startup_status_for_result(result, recovery_warning);
                    if status.phase == "ready" {
                        tracing::info!(
                            event = "app_startup",
                            stage = "database_open_and_migrate",
                            outcome = "succeeded"
                        );
                    } else {
                        tracing::error!(
                            event = "app_startup",
                            stage = "database_open_and_migrate",
                            outcome = "failed",
                            error_code = status
                                .error_code
                                .as_deref()
                                .unwrap_or("database_startup_failed")
                        );
                    }
                    store = connected_store;
                    startup_status = status;
                }
                Err(_) => {
                    tracing::error!(
                        event = "app_startup",
                        stage = "application_data_directory",
                        outcome = "failed",
                        error_code = "application_data_directory_unavailable"
                    );
                }
            }

            tracing::info!(
                event = "app_startup",
                stage = "ui_host_ready",
                outcome = "succeeded",
                vault_ready = startup_status.phase == "ready"
            );
            app.manage(AppState {
                store,
                startup_status,
                lifecycle_session: Mutex::new(lifecycle_session),
                trace_guard: Mutex::new(trace_guard),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_startup_status,
            list_projects,
            create_project,
            list_missions,
            create_mission,
            inspect_workspace,
            approve_mission,
            list_artifacts,
            list_evidence
        ])
        .build(tauri::generate_context!())
        .expect("Rocksoul Tauri runtime could not be built");

    app.run(|app_handle, event| {
        if matches!(event, tauri::RunEvent::Exit) {
            tracing::info!(
                event = "app_shutdown",
                stage = "runtime_exit",
                outcome = "started"
            );

            if let Some(state) = app_handle.try_state::<AppState>() {
                if let Some(store) = state.store.as_ref() {
                    tauri::async_runtime::block_on(store.close());
                }

                match state.lifecycle_session.lock() {
                    Ok(mut guard) => {
                        if let Some(session) = guard.take() {
                            match session.finish_cleanly() {
                                Ok(()) => tracing::info!(
                                    event = "app_shutdown",
                                    stage = "session_marker",
                                    outcome = "clean"
                                ),
                                Err(_) => tracing::error!(
                                    event = "app_shutdown",
                                    stage = "session_marker",
                                    outcome = "failed",
                                    error_code = "clean_shutdown_marker_remove_failed"
                                ),
                            }
                        } else {
                            tracing::warn!(
                                event = "app_shutdown",
                                stage = "session_marker",
                                outcome = "unavailable"
                            );
                        }
                    }
                    Err(_) => tracing::error!(
                        event = "app_shutdown",
                        stage = "session_marker",
                        outcome = "failed",
                        error_code = "lifecycle_state_lock_failed"
                    ),
                }

                // Dropping the worker guard flushes queued JSON log records.
                if let Ok(mut guard) = state.trace_guard.lock() {
                    guard.take();
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_open_failure_never_exposes_a_store_or_ready_status() -> Result<(), Box<dyn std::error::Error>> {
        let temporary = tempfile::tempdir()?;
        let database_path_is_a_directory = temporary.path().join("not-a-database.sqlite3");
        std::fs::create_dir(&database_path_is_a_directory)?;

        let open_result =
            tauri::async_runtime::block_on(Store::connect(&database_path_is_a_directory));
        assert!(open_result.is_err(), "a directory cannot be opened as the SQLite database");

        let (store, status) = startup_status_for_result(
            open_result,
            Some("previous-session marker was present".to_string()),
        );

        assert!(store.is_none());
        assert_eq!(status.phase, "failed");
        assert_eq!(
            status.error_code.as_deref(),
            Some("database_connection_failed")
        );
        assert_eq!(
            status.recovery_warning.as_deref(),
            Some("previous-session marker was present")
        );
        Ok(())
    }
}
