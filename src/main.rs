#[macro_use]
extern crate lazy_static;

// 引入包模块
extern crate redis;

// use redis::{Client, RedisResult};
// use redis::Commands;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

// rust 教学 https://www.runoob.com/rust/rust-tutorial.html
// rust enum https://www.runoob.com/rust/rust-enum.html
// rust 所有权 https://www.runoob.com/rust/rust-ownership.html
// rust 泛型 https://www.runoob.com/rust/rust-generics.html
// rust Option<T>, Result<T,E>, unwrap panic!, ?
// rust |x:T| {} 匿名函数  |  闭包？ Stack closures, Managed closures @fn, Owned closures ~fn
// rust References(引用),borrowing(借用),&关键字,ref关键字,* https://www.jianshu.com/p/ac519d8c5ec9


// 静态全局变量宏
lazy_static! {
    // static ref CLI:RedisResult<Client> = redis::Client::open("redis://127.0.0.1/5");
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    // match 表达式
    match(_req.method(), _req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Hello World");
        },

        (&Method::POST, "/getDeviceStatus") => {
            *response.body_mut() = Body::from("getDeviceStatus");
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

#[tokio::main]
async fn main() {

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });
    let server = Server::bind(&addr).serve(make_svc);
    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

}
