use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use pushlock_lib::UserInfo;
use state::Context;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

mod args;
mod state;

struct Runtime {
    state: Mutex<Context>,
}

#[post("/lock")]
async fn lock(user_info: web::Json<UserInfo>, data: web::Data<Arc<Runtime>>) -> impl Responder {
    let mut state = data.state.lock().await;
    let user_info = user_info.into_inner();

    match state.lock(user_info.username) {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(lock_state) => HttpResponse::build(StatusCode::LOCKED).json(lock_state),
    }
}

#[post("/release")]
async fn unlock(user_info: web::Json<UserInfo>, data: web::Data<Arc<Runtime>>) -> impl Responder {
    let mut state = data.state.lock().await;
    let user_info = user_info.into_inner();

    match state.unlock(user_info.username) {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(lock_state) => HttpResponse::build(StatusCode::LOCKED).json(lock_state),
    }
}

#[get("/lock_state")]
async fn get_state(
    user_info: web::Json<UserInfo>,
    data: web::Data<Arc<Runtime>>,
) -> impl Responder {
    let state = data.state.lock().await;
    let user_info = user_info.into_inner();
    let lock_state = state.get_lock_status(&user_info.username);
    let lock_status = match lock_state.push_available {
        true => StatusCode::OK,
        false => StatusCode::LOCKED,
    };

    HttpResponse::build(lock_status).json(lock_state)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    flexi_logger::Logger::with_env_or_str("actix_web=trace")
        .start()
        .unwrap();
    let port: u16 = args::get_port();
    let runtime = Arc::new(Runtime {
        state: Default::default(),
    });

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));

    HttpServer::new(move || {
        App::new()
            .data(runtime.clone())
            .service(lock)
            .service(unlock)
            .service(get_state)
    })
    .bind(bind_addr)?
    .run()
    .await
}
