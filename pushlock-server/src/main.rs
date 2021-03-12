use actix_web::{get, http::StatusCode, post, web, App, HttpRequest, HttpResponse, HttpServer};
use state::Context;
use std::{net::IpAddr, sync::Arc};
use tokio::sync::Mutex;

mod args;
mod state;

#[derive(Default, Clone)]
struct Runtime {
    state: Arc<Mutex<Context>>,
}

async fn check_lock<F>(req: HttpRequest, data: web::Data<Runtime>, f: F) -> HttpResponse
where
    F: FnOnce(&mut Context, IpAddr),
{
    let mut state = data.state.lock().await;
    let requested_ip = req.peer_addr().unwrap().ip();

    if let Some(locked) = state.get_lock_status(&requested_ip) {
        return HttpResponse::build(StatusCode::LOCKED).json(locked);
    }

    f(&mut state, requested_ip);

    HttpResponse::Ok().finish()
}

#[post("/lock")]
async fn lock(req: HttpRequest, data: web::Data<Runtime>) -> HttpResponse {
    check_lock(req, data, |state, ip| {
        state.locks.insert(ip);
    })
    .await
}

#[post("/release")]
async fn unlock(req: HttpRequest, data: web::Data<Runtime>) -> HttpResponse {
    check_lock(req, data, |state, ip| {
        state.locks.remove(&ip);
    }).await
}

#[get("/is_locked")]
async fn get_state(req: HttpRequest, data: web::Data<Runtime>) -> HttpResponse {
    check_lock(req, data, |_state, _ip| ()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = args::get_port();
    let runtime = Runtime::default();

    HttpServer::new(move || {
        let runtime = web::Data::new(runtime.clone());

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
