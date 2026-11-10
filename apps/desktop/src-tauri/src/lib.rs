use std::fs;

use rocksoul_core::{Artifact, Evidence, InspectionResult, Mission, Project, Store};
use tauri::{Manager, State};

struct AppState {
    store: Store,
}

#[tauri::command]
async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    state.store.list_projects().await.map_err(|error| error.to_string())
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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_local_data_dir()?;
            fs::create_dir_all(&data_dir)?;
            let database_path = data_dir.join("rocksoul.sqlite3");

            let store = tauri::async_runtime::block_on(Store::connect(&database_path))
                .map_err(|error| std::io::Error::other(error.to_string()))?;

            app.manage(AppState { store });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_projects,
            create_project,
            list_missions,
            create_mission,
            inspect_workspace,
            list_artifacts,
            list_evidence
        ])
        .run(tauri::generate_context!())
        .expect("Rocksoul failed to start");
}
