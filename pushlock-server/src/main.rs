use actix_web::{
    get, http::StatusCode, post, web, web::Data, App, HttpResponse, HttpServer, Responder,
};
use pushlock_lib::UserInfo;
use state::Context;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

mod args;
mod state;

struct Runtime {
    state: Mutex<Context>,
}

async fn check_lock<F>(
    user_info: web::Json<UserInfo>,
    data: web::Data<Arc<Runtime>>,
    f: F,
) -> impl Responder
where
    F: FnOnce(&mut Context, String),
{
    let mut state = data.state.lock().await;
    let user_info = user_info.into_inner();
    let lock_state = state.get_lock_status(&user_info.username);

    let lock_status = if lock_state.push_available {
        StatusCode::OK
    } else {
        StatusCode::LOCKED
    };

    if lock_state.push_available {
        f(&mut state, user_info.username);
    }

    HttpResponse::build(lock_status).json(lock_state)
}

#[post("/lock")]
async fn lock(user_info: web::Json<UserInfo>, data: web::Data<Arc<Runtime>>) -> impl Responder {
    check_lock(user_info, data, |state, username| {
        state.locked_by = Some(username);
    })
    .await
}

#[post("/release")]
async fn unlock(user_info: web::Json<UserInfo>, data: web::Data<Arc<Runtime>>) -> impl Responder {
    check_lock(user_info, data, |state, _username| {
        state.locked_by.take();
    })
    .await
}

#[get("/lock_state")]
async fn get_state(
    user_info: web::Json<UserInfo>,
    data: web::Data<Arc<Runtime>>,
) -> impl Responder {
    check_lock(user_info, data, |_state, _username| ()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    flexi_logger::Logger::try_with_env_or_str("actix_web=trace")
        .unwrap()
        .start()
        .unwrap();
    let port: u16 = args::get_port();
    let runtime = Arc::new(Runtime {
        state: Default::default(),
    });

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(runtime.clone()))
            .service(lock)
            .service(unlock)
            .service(get_state)
    })
    .bind(bind_addr)?
    .run()
    .await
}
