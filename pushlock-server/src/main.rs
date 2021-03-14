use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use state::Context;
use std::{convert::Infallible, net::IpAddr, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

mod args;
mod state;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Default)]
struct Runtime {
    state: Mutex<Context>,
}

async fn check_lock<F>(ip: IpAddr, data: Arc<Runtime>, f: F) -> Result<Response<Body>, BoxError>
where
    F: FnOnce(&mut Context),
{
    let mut state = data.state.lock().await;

    if let Some(locked) = state.get_lock_status(&ip) {
        let bytes = serde_json::to_vec(&locked)?;
        return Ok(Response::builder()
            .status(StatusCode::LOCKED)
            .body(Body::from(bytes))?);
    }

    f(&mut state);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?)
}

// #[post("/lock")]
async fn lock(ip: IpAddr, data: Arc<Runtime>) -> Result<Response<Body>, BoxError> {
    check_lock(ip, data, |state| {
        state.locks.insert(ip);
    })
    .await
}

// #[post("/release")]
async fn unlock(ip: IpAddr, data: Arc<Runtime>) -> Result<Response<Body>, BoxError> {
    check_lock(ip, data, |state| {
        state.locks.remove(&ip);
    })
    .await
}

// #[get("/is_locked")]
async fn get_state(ip: IpAddr, data: Arc<Runtime>) -> Result<Response<Body>, BoxError> {
    check_lock(ip, data, |_state| ()).await
}

async fn handle_request(
    ip: IpAddr,
    req: Request<Body>,
    runtime: Arc<Runtime>,
) -> Result<Response<Body>, BoxError> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/lock") => lock(ip, runtime).await,
        (&Method::POST, "/release") => unlock(ip, runtime).await,
        (&Method::GET, "/is_locked") => get_state(ip, runtime).await,
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

    let make_service = make_service_fn(move |client: &AddrStream| {
        let ip = client.remote_addr().ip();
        let runtime = runtime.clone();
        async move {
            // This is the request handler.
            Ok::<_, Infallible>(service_fn(move |req| {
                let runtime = runtime.clone();
                handle_request(ip, req, runtime)
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
