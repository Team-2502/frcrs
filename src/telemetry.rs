use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use axum::body::Body;
use axum::http::{header, HeaderValue, Method, Response, StatusCode};
use axum::response::IntoResponse;
use tokio::sync::{Mutex, RwLock};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use lazy_static::lazy_static;
use tokio::fs;
use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use tokio::time::sleep;
use tower_http::cors::{Any, CorsLayer};
use crate::ctre::{ControlMode, Talon};
use crate::{observe_user_program_starting, refresh_data};
use crate::input::RobotState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectorData {
    options: Vec<String>,
    selected: String,
}

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
            .route("/status", get(status_check))
            .route("/telemetry", post(update_telemetry).get(get_telemetry))
            .route("/", get(frontend))
            .route("/*path", get(frontend))
            .route("/telemetry/:key", get(get_telemetry_value).put(set_telemetry_value))
            .route("/telemetry_layout", post(save_layout).get(load_layout))
            .layer(Extension(TELEMETRY_STATE.clone()))
            .layer(CorsLayer::very_permissive());

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

    pub async fn put_selector(key: &str, options: Vec<String>) {
        let selector_data = SelectorData {
            options: options.clone(),
            selected: options.first()
                .map(|s| s.clone())
                .unwrap_or_else(|| "".to_string()),
        };

        let json = serde_json::to_string(&selector_data).unwrap();
        Self::put_string(key, json).await;
    }

    pub async fn get_selection(key: &str) -> Option<String> {
        if let Some(json) = Self::get(key).await {
            serde_json::from_str::<SelectorData>(&json)
                .ok()
                .map(|data| data.selected)
        } else {
            None
        }
    }

    pub async fn get_options(key: &str) -> Option<Vec<String>> {
        if let Some(json) = Self::get(key).await {
            serde_json::from_str::<SelectorData>(&json)
                .ok()
                .map(|data| data.options)
        } else {
            None
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

async fn status_check() -> impl IntoResponse {
    "OK"
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
            .status(StatusCode::OK)
            .body(Body::new("null".into_response()))
            .unwrap()
            .into_response(),
    }
}

async fn set_telemetry_value(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(key): Path<String>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let state = state.lock().await;
    let mut telemetry_data = state.telemetry_data.write().await;

    let value = if let Some(selection) = payload.get("selected") {
        if let Some(existing) = telemetry_data.iter().find(|data| data.key == key) {
            if let Ok(mut selector) = serde_json::from_str::<SelectorData>(&existing.value) {
                selector.selected = selection.as_str().unwrap_or("").to_string();
                serde_json::to_string(&selector).unwrap()
            } else {
                payload.to_string()
            }
        } else {
            payload.to_string()
        }
    } else {
        payload.to_string()
    };

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

async fn save_layout(Json(layout): Json<Value>) -> impl IntoResponse {
    if let Ok(layout_str) = serde_json::to_string_pretty(&layout) {
        if let Err(e) = fs::write("dashboard_layout.json", layout_str).await {
            eprintln!("Error saving layout: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to save layout"))
                .unwrap();
        }
    }

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Layout saved"))
        .unwrap()
}

async fn load_layout() -> impl IntoResponse {
    match fs::read_to_string("dashboard_layout.json").await {
        Ok(content) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(content))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("{}"))
            .unwrap(),
    }
}

#[test]
fn telemetry() {
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();

    let controller = local.run_until(async {
        Telemetry::init(5807);

        Telemetry::put_number("number test", 42.0).await;
        Telemetry::put_string("string test", "hello".to_string()).await;
        Telemetry::put_vec("vec test", vec![1, 2, 3]).await;
        Telemetry::put_selector("selector test", vec!["one".to_string(), "two".to_string(), "three".to_string()]).await;

        loop {
            if let Some(selected) = Telemetry::get_selection("selector test").await {
                println!("Current mode: {}", selected);
            } else {
                eprintln!("Failed to get from selector")
            }

            sleep(Duration::from_secs(5)).await;
        }
    });

    executor.block_on(controller);
}