mod lifecycle;

use std::fs;

use lifecycle::LifecycleMarker;
use rocksoul_core::{Artifact, Evidence, InspectionResult, Mission, Project, Store};
use serde::Serialize;
use tauri::{Manager, State};

struct AppState {
    store: Store,
    lifecycle: LifecycleMarker,
    startup: StartupDiagnostics,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct StartupDiagnostics {
    previous_unclean_exit: bool,
}

#[tauri::command]
fn get_startup_diagnostics(state: State<'_, AppState>) -> StartupDiagnostics {
    tracing::info!(event = "ui_ready", stage = "startup_diagnostics_command");
    state.startup.clone()
}

#[tauri::command]
async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    state
        .store
        .list_projects()
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn create_project(
    name: String,
    root_path: String,
    state: State<'_, AppState>,
) -> Result<Project, String> {
    state
        .store
        .create_project(&name, &root_path)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn list_missions(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Mission>, String> {
    state
        .store
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
    state
        .store
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
    state
        .store
        .inspect_workspace(&project_id, &mission_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn list_artifacts(
    mission_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Artifact>, String> {
    state
        .store
        .list_artifacts(&mission_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn list_evidence(
    mission_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Evidence>, String> {
    state
        .store
        .list_evidence(&mission_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn approve_mission(
    mission_id: String,
    state: State<'_, AppState>,
) -> Result<Mission, String> {
    state
        .store
        .approve_mission(&mission_id)
        .await
        .map_err(|error| error.to_string())
}

fn initialize_structured_logging() {
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_ansi(false)
        .with_current_span(true)
        .with_span_list(true)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}

pub fn run() {
    initialize_structured_logging();
    tracing::info!(event = "application_starting", stage = "runtime_bootstrap");

    let application = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_dir = app.path().app_local_data_dir().map_err(|_| {
                tracing::error!(
                    event = "startup_failed",
                    stage = "resolve_app_data",
                    error_class = "app_data_path_unavailable"
                );
                std::io::Error::other("app-local data directory unavailable")
            })?;
            fs::create_dir_all(&data_dir)?;

            // Create the marker before opening SQLite, so a failed setup remains visible.
            let lifecycle = LifecycleMarker::begin(&data_dir).map_err(|_| {
                tracing::error!(
                    event = "startup_failed",
                    stage = "lifecycle_marker",
                    error_class = "lifecycle_marker_unavailable"
                );
                std::io::Error::other("could not initialize the lifecycle marker")
            })?;
            if lifecycle.previous_unclean_exit() {
                tracing::warn!(event = "unclean_previous_exit_detected", stage = "startup_review");
            } else {
                tracing::info!(event = "clean_start_detected", stage = "startup_review");
            }

            let database_path = data_dir.join("rocksoul.sqlite3");
            tracing::info!(event = "startup_phase", stage = "database_connect_and_migrate");
            let store = tauri::async_runtime::block_on(Store::connect(&database_path))
                .map_err(|_| {
                    tracing::error!(
                        event = "startup_failed",
                        stage = "database_connect_and_migrate",
                        error_class = "vault_open_or_migration_failed"
                    );
                    std::io::Error::other("local vault could not be opened")
                })?;

            let startup = StartupDiagnostics {
                previous_unclean_exit: lifecycle.previous_unclean_exit(),
            };
            app.manage(AppState { store, lifecycle, startup });
            tracing::info!(event = "vault_connected", stage = "database_ready");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_startup_diagnostics,
            list_projects,
            create_project,
            list_missions,
            create_mission,
            inspect_workspace,
            approve_mission,
            list_artifacts,
            list_evidence
        ])
        .build(tauri::generate_context!());

    let application = match application {
        Ok(application) => application,
        Err(_) => {
            tracing::error!(
                event = "application_start_failed",
                stage = "tauri_builder",
                error_class = "tauri_setup_failed"
            );
            return;
        }
    };

    application.run(|app_handle, event| {
        if let tauri::RunEvent::ExitRequested { .. } = event {
            tracing::info!(event = "application_shutdown_started", stage = "close_database");
            let state = app_handle.state::<AppState>();
            tauri::async_runtime::block_on(state.store.close());

            match state.lifecycle.mark_clean_shutdown() {
                Ok(()) => tracing::info!(
                    event = "application_shutdown_complete",
                    stage = "clean_shutdown_marker_removed"
                ),
                Err(_) => tracing::error!(
                    event = "application_shutdown_incomplete",
                    stage = "clean_shutdown_marker_removal",
                    error_class = "lifecycle_marker_cleanup_failed"
                ),
            }
        }
    });
}
