use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use axum::body::Body;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use tokio::sync::{Mutex, RwLock};
use serde_json::json;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelemetryData {
    key: String,
    value: String,
}

pub struct AppState {
    telemetry_data: RwLock<Vec<TelemetryData>>,
}

lazy_static! {
    static ref TELEMETRY_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState {
        telemetry_data: RwLock::new(vec![]),
    }));
}

pub struct Telemetry;

impl Telemetry {
    pub fn init(port: u16) {
        let app = Router::new()
            .route("/telemetry", post(update_telemetry).get(get_telemetry))
            .route("/", get(frontend))
            .route("/*path", get(frontend))
            .route("/telemetry/:key", get(get_telemetry_value).put(set_telemetry_value))
            .layer(Extension(TELEMETRY_STATE.clone()));

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        println!("Listening on {}", addr);

        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    }

    pub async fn put_number(key: &str, value: f64) {
        Self::put_string(key, value.to_string()).await;
    }

    pub async fn put_string(key: &str, value: String) {
        let state = TELEMETRY_STATE.lock().await;
        let mut telemetry_data = state.telemetry_data.write().await;

        if let Some(existing) = telemetry_data.iter_mut().find(|data| data.key == key) {
            existing.value = value;
        } else {
            telemetry_data.push(TelemetryData {
                key: key.to_string(),
                value,
            });
        }
    }

    pub async fn put_vec<T: Serialize>(key: &str, values: Vec<T>) {
        let json_values = serde_json::to_string(&values).unwrap();
        let state = TELEMETRY_STATE.lock().await;
        let mut telemetry_data = state.telemetry_data.write().await;

        if let Some(existing) = telemetry_data.iter_mut().find(|data| data.key == key) {
            existing.value = json_values;
        } else {
            telemetry_data.push(TelemetryData {
                key: key.to_string(),
                value: json_values,
            });
        }
    }

    pub async fn get(key: &str) -> Option<String> {
        let state = TELEMETRY_STATE.lock().await;
        let telemetry_data = state.telemetry_data.read().await;

        telemetry_data.iter()
            .find(|data| data.key == key)
            .map(|data| data.value.clone())
    }
}

async fn frontend(Path(path): Path<Vec<String>>) -> impl IntoResponse {
    let mut path = path.join("/");
    let mut path = path.trim_start_matches('/');

    if path.is_empty() {
        path = "index.html";
    }

    let mime_type = mime_guess::from_path(path.clone()).first_or_text_plain();

    let dir = PathBuf::from(format!("/home/lvuser/frontend/{}", path));

    match File::open(dir.clone()).await {
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).await.unwrap();

            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(Body::from(contents))
                .unwrap()
        },
    }
}

async fn update_telemetry(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(payload): Json<TelemetryData>,
) -> String {
    let state = state.lock().await;
    state.telemetry_data.write().await.push(payload);
    json!({"status": "success"}).to_string()
}

async fn get_telemetry(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> Json<Vec<TelemetryData>> {
    let state = state.lock().await;
    let telemetry_data = state.telemetry_data.read().await.clone();
    Json(telemetry_data)
}

async fn get_telemetry_value(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let state = state.lock().await;
    let telemetry_data = state.telemetry_data.read().await;

    match telemetry_data.iter().find(|data| data.key == key) {
        Some(data) => data.clone().value.into_response(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
            .into_response(),
    }
}

async fn set_telemetry_value(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(key): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let state = state.lock().await;
    let mut telemetry_data = state.telemetry_data.write().await;

    let value = payload.to_string();
    if let Some(existing) = telemetry_data.iter_mut().find(|data| data.key == key) {
        existing.value = value;
    } else {
        telemetry_data.push(TelemetryData {
            key: key.clone(),
            value,
        });
    }

    json!({"status": "success"}).to_string().into_response()
}