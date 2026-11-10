mod db;
mod scanner;

use db::{Artifact, Database, Evidence, Mission, Project};
use scanner::{InspectionReport, inspect_directory};
use std::path::PathBuf;
use tauri::{Manager, State};

#[tauri::command]
async fn list_projects(database: State<'_, Database>) -> Result<Vec<Project>, String> {
    database.list_projects().await
}

#[tauri::command]
async fn create_project(name: String, database: State<'_, Database>) -> Result<Project, String> {
    database.create_project(name).await
}

#[tauri::command]
async fn list_missions(project_id: String, database: State<'_, Database>) -> Result<Vec<Mission>, String> {
    database.list_missions(&project_id).await
}

#[tauri::command]
async fn create_mission(
    project_id: String,
    title: String,
    brief: String,
    database: State<'_, Database>,
) -> Result<Mission, String> {
    database.create_mission(project_id, title, brief).await
}

#[tauri::command]
async fn start_mission(id: String, database: State<'_, Database>) -> Result<Mission, String> {
    database.start_mission(&id).await
}

#[tauri::command]
async fn save_artifact(
    mission_id: String,
    title: String,
    content: String,
    database: State<'_, Database>,
) -> Result<Artifact, String> {
    database.save_artifact(&mission_id, title, content).await
}

#[tauri::command]
async fn list_artifacts(mission_id: String, database: State<'_, Database>) -> Result<Vec<Artifact>, String> {
    database.list_artifacts(&mission_id).await
}

#[tauri::command]
async fn complete_mission(id: String, database: State<'_, Database>) -> Result<Mission, String> {
    database.complete_mission(&id).await
}

#[tauri::command]
async fn list_evidence(project_id: String, database: State<'_, Database>) -> Result<Vec<Evidence>, String> {
    database.list_evidence(&project_id).await
}

#[tauri::command]
async fn inspect_workspace(
    project_id: String,
    path: String,
    database: State<'_, Database>,
) -> Result<InspectionReport, String> {
    let report = tauri::async_runtime::spawn_blocking(move || {
        inspect_directory(&PathBuf::from(path))
    })
    .await
    .map_err(|error| format!("Workspace inspection task failed: {error}"))??;

    database.save_inspection(&project_id, &report).await?;
    Ok(report)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let database_path = data_dir.join("rocksoul.sqlite");
            let database = tauri::async_runtime::block_on(Database::connect(database_path))
                .map_err(std::io::Error::other)?;
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_projects,
            create_project,
            list_missions,
            create_mission,
            start_mission,
            save_artifact,
            list_artifacts,
            complete_mission,
            list_evidence,
            inspect_workspace
        ])
        .run(tauri::generate_context!())
        .expect("error while running Rocksoul");
}
