use crate::download::{DownloadProgress, DownloadRequest, DownloadState};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
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

    guard.cancelled.remove(&id);
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
    guard.cancelled.insert(id.clone());
    guard.active_downloads.remove(&id);
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
pub async fn delete_download_file(
    state: State<'_, Arc<Mutex<DownloadState>>>,
    id: String,
    file_path: Option<String>,
) -> Result<(), String> {
    let file_path = if file_path.is_some() {
        file_path
    } else {
        let guard = state.lock().await;
        guard
            .completed_downloads
            .iter()
            .find(|d| d.id == id)
            .and_then(|d| d.file_path.clone())
    };

    if let Some(path) = file_path {
        let file = std::path::PathBuf::from(&path);
        if file.exists() {
            tokio::fs::remove_file(&file)
                .await
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }
    }

    let mut guard = state.lock().await;
    guard.completed_downloads.retain(|d| d.id != id);
    Ok(())
}

#[tauri::command]
pub async fn open_download_folder(path: String) -> Result<(), String> {
    opener::open(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reveal_download_file(path: String) -> Result<(), String> {
    opener::reveal(std::path::PathBuf::from(path)).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_download_dir(
    state: State<'_, Arc<Mutex<DownloadState>>>,
) -> Result<String, String> {
    let guard = state.lock().await;
    Ok(guard.get_download_dir().to_string_lossy().to_string())
}

#[tauri::command]
pub async fn set_download_dir(
    state: State<'_, Arc<Mutex<DownloadState>>>,
    path: String,
) -> Result<String, String> {
    let dir = std::path::PathBuf::from(path);
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create download directory: {}", e))?;

    let mut guard = state.lock().await;
    guard.set_download_dir(dir.clone());
    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn choose_download_dir(
    app: AppHandle,
    state: State<'_, Arc<Mutex<DownloadState>>>,
) -> Result<Option<String>, String> {
    let current_dir = {
        let guard = state.lock().await;
        guard.get_download_dir()
    };

    let selected = app
        .dialog()
        .file()
        .set_title("Select Download Folder")
        .set_directory(current_dir)
        .blocking_pick_folder()
        .and_then(|file_path| file_path.into_path().ok());

    let Some(path) = selected else {
        return Ok(None);
    };

    tokio::fs::create_dir_all(&path)
        .await
        .map_err(|e| format!("Failed to create download directory: {}", e))?;

    let mut guard = state.lock().await;
    guard.set_download_dir(path.clone());
    Ok(Some(path.to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn get_app_data_path(app: AppHandle) -> Result<String, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

/// Proxy wallpaper image through Rust backend to bypass CDN hotlinking (403)
/// Downloads the image to local cache dir and returns the local file path.
#[tauri::command]
pub async fn fetch_wallpaper_image(
    app: AppHandle,
    path: String,
) -> Result<String, String> {
    let cache_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("cache")
        .join("images");

    tokio::fs::create_dir_all(&cache_dir)
        .await
        .map_err(|e| format!("Failed to create cache dir: {}", e))?;

    let file_name = std::path::Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "wallpaper.jpg".into());

    let local_path = cache_dir.join(&file_name);

    // Return cached file if it exists
    if local_path.exists() {
        return Ok(local_path.to_string_lossy().to_string());
    }

    let cdn_base = "https://w.wallhaven.cc/full/";
    let url = if path.starts_with("https://") || path.starts_with("http://") {
        path.clone()
    } else {
        format!("{}{}", cdn_base, path.trim_start_matches('/'))
    };

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(&url)
        .header("Referer", "https://wallhaven.cc/")
        .send()
        .await
        .map_err(|e| format!("Image request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "CDN Error: {} {}",
            response.status().as_u16(),
            response.status().canonical_reason().unwrap_or("Unknown")
        ));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read image bytes: {}", e))?;

    tokio::fs::write(&local_path, &bytes)
        .await
        .map_err(|e| format!("Failed to write image file: {}", e))?;

    Ok(local_path.to_string_lossy().to_string())
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
