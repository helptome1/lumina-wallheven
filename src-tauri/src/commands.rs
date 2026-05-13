use crate::download::{DownloadProgress, DownloadRequest, DownloadState};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn download_image(
    app: AppHandle,
    state: State<'_, Arc<Mutex<DownloadState>>>,
    url: String,
    id: String,
    file_name: String,
    resolution: String,
) -> Result<(), String> {
    let mut guard = state.lock().await;

    if guard.active_downloads.contains_key(&id) {
        return Err("Already downloading".into());
    }

    let download_dir = guard.get_download_dir();

    let request = DownloadRequest {
        id: id.clone(),
        url,
        file_name,
        resolution,
        download_dir,
    };

    guard.active_downloads.insert(id.clone(), None);
    drop(guard);

    // Spawn download in background
    let state_clone = app.state::<Arc<Mutex<DownloadState>>>().inner().clone();
    let app_clone = app.clone();
    let id_clone = id.clone();

    tokio::spawn(async move {
        let result = crate::download::execute_download(
            DownloadRequest {
                id: id_clone.clone(),
                url: request.url,
                file_name: request.file_name,
                resolution: request.resolution,
                download_dir: request.download_dir,
            },
            app_clone.clone(),
        )
        .await;

        let mut guard = state_clone.lock().await;
        match result {
            Ok(path) => {
                guard.active_downloads.remove(&id_clone);
                guard.completed_downloads.push(DownloadProgress {
                    id: id_clone,
                    received_bytes: 0,
                    total_bytes: 0,
                    speed_bytes: 0,
                    state: "done".into(),
                    file_path: Some(path),
                    file_name: None,
                    resolution: None,
                    thumbnail: None,
                });
            }
            Err(e) => {
                guard.active_downloads.remove(&id_clone);
                let _ = app_clone.emit(
                    "download-error",
                    serde_json::json!({
                        "id": id_clone,
                        "error": e,
                    }),
                );
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn cancel_download(
    state: State<'_, Arc<Mutex<DownloadState>>>,
    id: String,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    guard.cancelled.insert(id);
    Ok(())
}

#[tauri::command]
pub async fn get_downloads(
    state: State<'_, Arc<Mutex<DownloadState>>>,
) -> Result<Vec<DownloadProgress>, String> {
    let guard = state.lock().await;
    Ok(guard.completed_downloads.clone())
}

#[tauri::command]
pub async fn clear_download(
    state: State<'_, Arc<Mutex<DownloadState>>>,
    id: String,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    guard.completed_downloads.retain(|d| d.id != id);
    Ok(())
}

#[tauri::command]
pub async fn open_download_folder(path: String) -> Result<(), String> {
    opener::open(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_app_data_path(app: AppHandle) -> Result<String, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

/// Proxy wallhaven.cc API requests through Rust backend to bypass CORS
#[tauri::command]
pub async fn fetch_wallhaven_api(
    endpoint: String,
    params: std::collections::HashMap<String, String>,
) -> Result<serde_json::Value, String> {
    let base_url = "https://wallhaven.cc/api/v1";
    let url = format!("{}{}", base_url, endpoint);

    let client = reqwest::Client::builder()
        .user_agent("Wallhaven-Desktop/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(&url)
        .query(&params)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "API Error: {} {}",
            response.status().as_u16(),
            response.status().canonical_reason().unwrap_or("Unknown")
        ));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(json)
}
