use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use serde::Deserialize;
use state::Context;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

mod args;
mod state;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Default)]
struct Runtime {
    state: Mutex<Context>,
}

#[derive(Deserialize)]
struct UserInfo {
    username: String,
}

async fn lock(username: String, data: Arc<Runtime>) -> Result<Response<Body>, BoxError> {
    let mut state = data.state.lock().await;

    let lock_state = state.get_lock_status(&username);

    if !lock_state.push_available {
        let bytes = serde_json::to_vec(&lock_state)?;
        return Ok(Response::builder()
            .status(StatusCode::LOCKED)
            .body(Body::from(bytes))?);
    }

    state.locked_by = Some(username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?)
}

async fn unlock(username: String, data: Arc<Runtime>) -> Result<Response<Body>, BoxError> {
    let mut state = data.state.lock().await;

    let lock_state = state.get_lock_status(&username);

    if !lock_state.push_available {
        let bytes = serde_json::to_vec(&lock_state)?;
        return Ok(Response::builder()
            .status(StatusCode::LOCKED)
            .body(Body::from(bytes))?);
    }

    state.locked_by.take();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?)
}

async fn get_state(username: String, data: Arc<Runtime>) -> Result<Response<Body>, BoxError> {
    let state = data.state.lock().await;
    let locked = state.get_lock_status(&username);
    let bytes = serde_json::to_vec(&locked)?;

    if !locked.push_available {
        let bytes = serde_json::to_vec(&locked)?;
        return Ok(Response::builder()
            .status(StatusCode::LOCKED)
            .body(Body::from(bytes))?);
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(bytes))?)
}

async fn handle_request(
    mut req: Request<Body>,
    runtime: Arc<Runtime>,
) -> Result<Response<Body>, BoxError> {
    let body = req.body_mut();
    let bytes = hyper::body::to_bytes(body).await?;
    let user_info: UserInfo = serde_json::from_slice(&bytes)?;
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/lock") => lock(user_info.username, runtime).await,
        (&Method::POST, "/release") => unlock(user_info.username, runtime).await,
        (&Method::GET, "/lock_state") => get_state(user_info.username, runtime).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())?),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = args::get_port();
    let runtime = Arc::new(Runtime::default());

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));

    let make_service = make_service_fn(move |_client: &AddrStream| {
        let runtime = runtime.clone();
        async move {
            // This is the request handler.
            Ok::<_, Infallible>(service_fn(move |req| {
                let runtime = runtime.clone();
                handle_request(req, runtime)
            }))
        }
    });

    let server = Server::bind(&bind_addr).serve(make_service);

    println!("Starting server on http://{}/", bind_addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
