use actix_web::{get, http::StatusCode, post, web, App, HttpRequest, HttpResponse, HttpServer};
use clap::Arg;
use std::{collections::HashSet, net::IpAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Default, Clone)]
struct LockState {
    locks: HashSet<IpAddr>,
}

impl LockState {
    fn is_locked(&self, requested_ip: &IpAddr) -> bool {
        self.locks.contains(requested_ip)
    }

    fn has_other_locks(&self, requested_ip: &IpAddr) -> bool {
        self.locks.iter().any(|l| l != requested_ip)
    }
}

#[derive(Default, Clone)]
struct Runtime {
    state: Arc<Mutex<LockState>>,
}

#[post("/lock")]
async fn lock(req: HttpRequest, data: web::Data<Runtime>) -> HttpResponse {
    let mut state = data.state.lock().await;
    let requested_ip = req.peer_addr().unwrap().ip();

    if state.is_locked(&requested_ip) {
        return HttpResponse::new(StatusCode::LOCKED);
    }

    state.locks.insert(requested_ip);

    HttpResponse::new(StatusCode::OK)
}

#[post("/release")]
async fn unlock(req: HttpRequest, data: web::Data<Runtime>) -> HttpResponse {
    let mut state = data.state.lock().await;
    let requested_ip = req.peer_addr().unwrap().ip();

    state.locks.remove(&requested_ip);

    HttpResponse::new(StatusCode::OK)
}

#[get("/is_locked")]
async fn get_state(req: HttpRequest, data: web::Data<Runtime>) -> HttpResponse {
    let state = data.state.lock().await;
    let requested_ip = req.peer_addr().unwrap().ip();

    if state.has_other_locks(&requested_ip) {
        HttpResponse::new(StatusCode::LOCKED)
    } else {
        HttpResponse::new(StatusCode::OK)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = clap::App::new("pushlock-server")
        .arg(
            Arg::new("port")
                .takes_value(true)
                .required(true)
                .default_value("8080"),
        )
        .get_matches();

    let port: u16 = app
        .value_of("port")
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let runtime = Runtime::default();

    let state = runtime.clone();

    HttpServer::new(move || {
        let runtime = web::Data::new(state.clone());

        App::new()
            .app_data(runtime)
            .service(lock)
            .service(unlock)
            .service(get_state)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
