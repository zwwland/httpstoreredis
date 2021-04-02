#[macro_use]
extern crate lazy_static;

// 引入包模块
// extern crate redis;
// extern crate redis as new_redis;
// use futures::prelude::*;
// use redis::Commands;
// use redis::AsyncCommands;
// use redis::{RedisResult, RedisError, Connection};

extern crate deadpool_redis;
extern crate json;

use deadpool_redis::{cmd, Config, Pool};
// use deadpool_redis::redis::FromRedisValue;

use json::{object};

use std::convert::Infallible;
use std::net::SocketAddr;
// use std::sync::Mutex;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Method, Request, Response, Server, StatusCode};

// rust 教学 https://www.runoob.com/rust/rust-tutorial.html
// rust enum https://www.runoob.com/rust/rust-enum.html
// rust 所有权 https://www.runoob.com/rust/rust-ownership.html
// rust 泛型 https://www.runoob.com/rust/rust-generics.html
// rust Option<T>, Result<T,E>, unwrap panic!, ?
// rust |x:T| {} 匿名函数  |  闭包？ Stack closures, Managed closures @fn, Owned closures ~fn
// rust References(引用),borrowing(借用),&关键字,ref关键字,* https://www.jianshu.com/p/ac519d8c5ec9
// STRUCT 1->struct Color (u8,u8,u8); 2->struct Color {r:u8,g:u8,b:u8}

// 静态全局变量宏
lazy_static! {
    #[derive(Debug)]
    static ref POOL:Pool = {
        let mut cfg = Config::default();
        cfg.url  = Some("redis://192.168.3.150:6379/5".to_string());
        let pool = cfg.create_pool().unwrap();
        // let client = redis::Client::open("redis://127.0.0.1:6379/5").unwrap();
        // let x = client.get_connection().unwrap();
        // let x:Mutex<redis::Connection> = Mutex::new(client.get_connection().unwrap());
        // x
        pool
    };
}
// static Conn:redis::Connection;

async fn hello_world(mut _req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (_req.method(), _req.uri().path()) {
        (&Method::GET, "/") => {
            println!("{:?}", &POOL);
            {
                let mut conn = POOL.get().await.unwrap();
                cmd("SET")
                    .arg(&["hello", "199"])
                    .execute_async(&mut conn)
                    .await
                    .unwrap();
            }
            *response.body_mut() = Body::from("Hello World");
        }

        (&Method::POST, "/getDeviceStatus") => {
            {
                let buffer = body::to_bytes(_req.body_mut()).await.unwrap();
                let bd = String::from_utf8_lossy(&buffer);
                let obj = json::parse(&bd).unwrap();
                let ret = object! {
                    name1: obj["name"].as_str(),
                    time1: obj["time"].as_str()
                };

                let mut conn = POOL.get().await.unwrap();
                cmd("SET")
                    .arg(&["hello", &ret.dump()])
                    .execute_async(&mut conn)
                    .await
                    .unwrap();
            }
            *response.body_mut() = Body::from("getDeviceStatus");
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
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
