//! [hyper] echo server
//!
//! [hyper]: https://hyper.rs/guides/server/echo/
use std::{convert::Infallible, net::SocketAddr, str::FromStr};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or(String::from("127.0.0.1:8088"));
    Runtime::new()?.block_on(server(addr))
}

async fn server(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(echo)) });
    let addr = SocketAddr::from_str(&addr)?;
    let server = Server::bind(&addr).serve(make_svc);
    server.await.map_err(|err| err.into())
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut resp = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *resp.body_mut() = Body::from("Try GETing data from /\n");
        }
        (&Method::POST, "/echo") => {
            *resp.body_mut() = req.into_body();
        }
        _ => {
            *resp.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    Ok(resp)
}
