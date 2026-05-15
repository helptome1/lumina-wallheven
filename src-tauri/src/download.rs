use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct DownloadRequest {
    pub id: String,
    pub url: String,
    pub file_name: String,
    pub resolution: String,
    pub download_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DownloadProgress {
    pub id: String,
    pub received_bytes: u64,
    pub total_bytes: u64,
    pub speed_bytes: u64,
    pub state: String,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub resolution: Option<String>,
    pub thumbnail: Option<String>,
}

pub struct DownloadState {
    pub active_downloads: HashMap<String, Option<PathBuf>>,
    pub completed_downloads: Vec<DownloadProgress>,
    pub cancelled: HashSet<String>,
    download_dir: PathBuf,
}

impl DownloadState {
    pub fn new() -> Self {
        Self {
            active_downloads: HashMap::new(),
            completed_downloads: Vec::new(),
            cancelled: HashSet::new(),
            download_dir: dirs::download_dir().unwrap_or_else(|| PathBuf::from(".")),
        }
    }

    pub fn set_download_dir(&mut self, path: PathBuf) {
        self.download_dir = path;
    }

    pub fn get_download_dir(&self) -> PathBuf {
        self.download_dir.clone()
    }
}

pub async fn execute_download(
    request: DownloadRequest,
    app: AppHandle,
) -> Result<String, String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let cdn_url = if request.url.starts_with("https://") || request.url.starts_with("http://") {
        request.url.clone()
    } else {
        format!("https://w.wallhaven.cc/full/{}", request.url.trim_start_matches('/'))
    };

    let response = client
        .get(&cdn_url)
        .header("Referer", "https://wallhaven.cc/")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let total_size = response.content_length().unwrap_or(0);
    let file_path = request.download_dir.join(&request.file_name);

    // Create parent directories if needed
    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let mut file = tokio::fs::File::create(&file_path)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;

    let mut stream = response.bytes_stream();
    let mut received: u64 = 0;
    let mut prev_received: u64 = 0;
    let mut last_emit = Instant::now();

    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        // Check cancellation
        {
            let state = app.state::<Arc<Mutex<DownloadState>>>();
            let guard = state.lock().await;
            if guard.cancelled.contains(&request.id) {
                let _ = tokio::fs::remove_file(&file_path).await;
                return Err("Cancelled".into());
            }
        }

        let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Write error: {}", e))?;

        received += chunk.len() as u64;

        // Emit progress every 100ms
        let elapsed = last_emit.elapsed();
        if elapsed.as_millis() >= 100 {
            let speed = if elapsed.as_secs_f64() > 0.0 {
                ((received - prev_received) as f64 / elapsed.as_secs_f64()) as u64
            } else {
                0
            };
            prev_received = received;

            let _ = app.emit(
                "download-progress",
                DownloadProgress {
                    id: request.id.clone(),
                    received_bytes: received,
                    total_bytes: total_size,
                    speed_bytes: speed,
                    state: "downloading".into(),
                    file_path: Some(file_path.to_string_lossy().to_string()),
                    file_name: Some(request.file_name.clone()),
                    resolution: Some(request.resolution.clone()),
                    thumbnail: None,
                },
            );
            last_emit = Instant::now();
        }
    }

    // Finalize
    let _ = app.emit(
        "download-progress",
        DownloadProgress {
            id: request.id.clone(),
            received_bytes: received,
            total_bytes: total_size,
            speed_bytes: 0,
            state: "done".into(),
            file_path: Some(file_path.to_string_lossy().to_string()),
            file_name: Some(request.file_name.clone()),
            resolution: Some(request.resolution.clone()),
            thumbnail: None,
        },
    );

    Ok(file_path.to_string_lossy().to_string())
}
