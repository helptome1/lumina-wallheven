use crate::download::{DownloadProgress, DownloadRequest, DownloadState};
use std::path::{Path, PathBuf};
use std::process::Command;
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
    let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

/// Proxy wallpaper image through Rust backend to bypass CDN hotlinking (403)
/// Downloads the image to local cache dir and returns the local file path.
#[tauri::command]
pub async fn fetch_wallpaper_image(app: AppHandle, path: String) -> Result<String, String> {
    let local_path = cache_wallpaper_image(&app, &path).await?;
    Ok(local_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn set_desktop_wallpaper(app: AppHandle, path: String) -> Result<(), String> {
    let local_path = cache_wallpaper_image(&app, &path).await?;
    set_system_wallpaper(&local_path)
}

async fn cache_wallpaper_image(app: &AppHandle, path: &str) -> Result<PathBuf, String> {
    let cache_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("cache")
        .join("images");

    tokio::fs::create_dir_all(&cache_dir)
        .await
        .map_err(|e| format!("Failed to create cache dir: {}", e))?;

    let file_name = Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "wallpaper.jpg".into());

    let local_path = cache_dir.join(&file_name);

    // Return cached file if it exists
    if local_path.exists() {
        return Ok(local_path);
    }

    let cdn_base = "https://w.wallhaven.cc/full/";
    let url = if path.starts_with("https://") || path.starts_with("http://") {
        path.to_string()
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

    Ok(local_path)
}

#[cfg(target_os = "macos")]
fn set_system_wallpaper(path: &Path) -> Result<(), String> {
    let script = r#"
on run argv
  tell application "System Events"
    set picture of every desktop to item 1 of argv
  end tell
end run
"#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run osascript: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

#[cfg(target_os = "windows")]
fn set_system_wallpaper(path: &Path) -> Result<(), String> {
    let script = r#"
Add-Type -TypeDefinition 'using System.Runtime.InteropServices; public class Wallpaper { [DllImport("user32.dll", SetLastError=true)] public static extern bool SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni); }'
if (-not [Wallpaper]::SystemParametersInfo(20, 0, $args[0], 3)) { exit 1 }
"#;

    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

#[cfg(target_os = "linux")]
fn set_system_wallpaper(path: &Path) -> Result<(), String> {
    let uri = format!("file://{}", path.to_string_lossy().replace(' ', "%20"));
    let status = Command::new("gsettings")
        .args(["set", "org.gnome.desktop.background", "picture-uri", &uri])
        .status()
        .map_err(|e| format!("Failed to run gsettings: {}", e))?;

    if !status.success() {
        return Err("Failed to set GNOME wallpaper".into());
    }

    let _ = Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri-dark",
            &uri,
        ])
        .status();

    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn set_system_wallpaper(_path: &Path) -> Result<(), String> {
    Err("Setting desktop wallpaper is not supported on this platform".into())
}

/// Proxy wallhaven.cc API requests through Rust backend to bypass CORS.
/// API key is passed via X-API-Key header (recommended by Wallhaven docs).
#[tauri::command]
pub async fn fetch_wallhaven_api(
    endpoint: String,
    params: std::collections::HashMap<String, String>,
    api_key: Option<String>,
) -> Result<serde_json::Value, String> {
    let base_url = "https://wallhaven.cc/api/v1";
    let url = format!("{}{}", base_url, endpoint);

    let client = reqwest::Client::builder()
        .user_agent("Wallhaven-Desktop/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let mut request = client.get(&url).query(&params);

    if let Some(key) = &api_key {
        if !key.is_empty() {
            request = request.header("X-API-Key", key);
        }
    }

    let response = request
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
