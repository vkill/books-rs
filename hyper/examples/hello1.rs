//! [hyper] hello world server
//!
//! [hyper]: https://hyper.rs/guides/server/hello-world/
use std::{convert::Infallible, net::SocketAddr, str::FromStr};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or(String::from("127.0.0.1:8088"));
    Runtime::new()?.block_on(server(addr))
}

async fn server(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello)) });
    let addr = SocketAddr::from_str(&addr)?;
    let server = Server::bind(&addr).serve(svc);
    server.await.map_err(|err| err.into())
}

async fn hello(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World\n".into()))
}
