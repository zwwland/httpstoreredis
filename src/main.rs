#[macro_use]
extern crate lazy_static;
extern crate mut_static;
// #[macro_use]
extern crate clap;
extern crate deadpool_redis;
extern crate json;

use clap::{App, Arg};
use deadpool_redis::{cmd, Config, Pool};
use mut_static::MutStatic;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use std::convert::Infallible;
use std::net::SocketAddr;
// use std::sync::Mutex;
// use std::sync::Arc;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Method, Request, Response, Server, StatusCode};

lazy_static! {
    #[derive(Debug)]
    static ref POOL:MutStatic<Pool> = MutStatic::new();
    static ref METHOD:MutStatic<Method> = MutStatic::new();
    static ref CONTEXT:MutStatic<String> = MutStatic::new();
}
async fn hello_world(mut _req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    let method = METHOD.read().unwrap();
    let context = CONTEXT.read().unwrap();

    match (_req.method(), _req.uri().path()) {
        (method, context) => {
            let mut conn = POOL.read().unwrap().get().await.unwrap();
            let mut hasher = DefaultHasher::new();
            let buffer = body::to_bytes(_req.body_mut()).await.unwrap();
            let bd = String::from_utf8_lossy(&buffer);
            hasher.write(&bd.as_bytes());
            cmd("SET")
                .arg(&[&hasher.finish().to_string()[0..], &bd])
                .execute_async(&mut conn)
                .await
                .unwrap();
            *response.body_mut() = Body::from("Hello World");
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    let matches = App::new("httptoredis")
        .version("1.0")
        .author("zwwland <zwwland@gmail.com>")
        .about("httptoredis is a httpserver for listen the request to store to redis tools !")
        .arg(
            Arg::with_name("method")
                .short("m")
                .long("method")
                .help("Set the http request method")
                .default_value("GET")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("server_context")
                .short("c")
                .long("context")
                .help("Sets the context of the http server path to listen")
                .takes_value(true)
                .default_value("/")
                .required(false),
        )
        .arg(
            Arg::with_name("server_port")
                .short("p")
                .long("port")
                .help("Sets the port of http server to listen")
                .takes_value(true)
                .default_value("3000")
                .required(false),
        )
        .arg(
            Arg::with_name("redis_uri")
                .short("r")
                .long("redis")
                .help("Sets the redis uri")
                .takes_value(true)
                .default_value("redis://127.0.0.1:6379/0")
                .required(false),
        )
        .get_matches();

    let uri = matches
        .value_of("redis_uri")
        .unwrap_or("redis://127.0.0.1:6379/0");
    let method = matches.value_of("config").unwrap_or("GET");
    let server_context = matches.value_of("server_context").unwrap_or("/");
    let server_port = matches.value_of("server_port").unwrap_or("3000");

    CONTEXT.set(String::from(server_context)).unwrap();
    METHOD
        .set(Method::from_bytes(method.as_bytes()).unwrap())
        .unwrap();

    let mut cfg = Config::default();
    cfg.url = Some(uri.to_string());
    let pool = cfg.create_pool().unwrap();
    POOL.set(pool).unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let make_svc = make_service_fn(move |_conn| async move {
        // service_fn converts our function into a `Service`
        async move {
            Ok::<_, Infallible>(service_fn(hello_world))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    // Run this server for... forever!
    println!("[redis] {} connect success. ", uri);
    println!(
        "[http] listen at *:{}{} [{}]",
        server_port, server_context, method
    );
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
