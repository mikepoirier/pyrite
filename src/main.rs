use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

async fn greeting(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, World")))
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();

    let make_svc = make_service_fn(move |_conn| {
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                async move {
                    Ok::<_, Infallible>(greeting(req).await?)
                }
            }))
        }
    });

    let server = Server::bind(&addr)
    .serve(make_svc);

    println!("Server Running on {}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
