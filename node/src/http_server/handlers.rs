use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Method, Request, Response, Server};
use hyper::server::conn::Http;
use hyper::service::{make_service_fn, service_fn};
use tower::make::Shared;
use crate::Node;

use treemux::{middleware_fn, Treemux, RouterBuilder, Params, RequestExt};
use treemux::middlewares;
use crate::blockchain::blockmap::BlockMap;


type Key = String;

type HttpResponse = Result<Response<Body>, hyper::http::Error>;

// TODO: Assess whether we want to keep this, and potentially extend this pattern, or do away with it
// use once_cell::sync::Lazy;
// use std::sync::Mutex;
//
// static NODE: Lazy<Mutex<Node>> = Lazy::new(|| {});


pub async fn start_server(node: Node) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    let blocks = node.blocks.clone();
    let make_svc = make_service_fn(move |req| {
        let mut router = Treemux::builder();
        router.get("/", hello_world);
        // router.get("/blocks/:block", async move { get_block(req, &blocks).await });

        let router: Arc<Treemux> = Arc::new(router.into());


        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let router = router.clone();
                async move { router.serve(req).await }
            }))
        }
    });

    // Shared is a MakeService that produces services by cloning an inner service...
    // let make_service = Shared::new(service_fn(move |req| { handle_request(req, node.clone()) }));


    // let n = node.clone();
    //
    // let mut router = Treemux::builder();
    // // let mut router = router.middleware(middleware_fn(middlewares::log));
    // router.get("/", hello_world);
    // // let a = async { get_block(req, &node.clone()) };
    //
    //
    // let blocks = node.blocks.clone();
    // router.get("/blocks/:hash", move |req: Request<Body>| {
    //     async { get_block(req, &blocks).await }
    // });

    // Then bind and serve...
    let server = Server::bind(&addr)
        .serve(make_svc);

    // And now add a graceful shutdown signal...
    // let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("HTTP server started!");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// async fn handle_request(req: Request<Body>, node: Node) -> HttpResponse {
//     match (req.method(), req.uri().path()) {
//         (&Method::GET, "/") => hello_world(req),
//         (&Method::GET, "/balance") => get_balance(req, &node),
//         (&Method::GET, "/block") => get_block(req, &node),
//         _ => Ok(Response::builder().status(404).body("".into()).unwrap())
//     }
// }

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

fn get_query_params(req: &Request<Body>) -> HashMap<String, String> {
    req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new)
}

async fn hello_world(_req: Request<Body>) -> HttpResponse {
    Ok(Response::new("Hello World".into()))
}

async fn get_balance(req: Request<Body>, node: &Node) -> HttpResponse {
    let params = get_query_params(&req);

    match params.get("key") {
        Some(key) => {
            let balance = node.balance_manager.get_balance(key);
            Ok(Response::new(balance.to_string().into()))
        }
        None => Ok(Response::new("error: missing \"key\" query param".into()))
    }
}

async fn get_block(req: Request<Body>, blocks: &BlockMap) -> HttpResponse {
    let hash = req.params().get("hash").unwrap();

    if let Some(block) = blocks.get(&hash.to_owned()) {
        let data = serde_json::to_string(&block).unwrap();
        Ok(Response::new(data.into()))
    } else {
        Ok(Response::new("error: block not found".into()))
    }
}

// pub fn post_transaction(req: Request<Body>) -> HttpResponse {
//     let full_body = hyper::body::to_bytes(req.into_body()).await?;
//
//     // Iterate the full body in reverse order and collect into a new Vec.
//     let reversed = full_body.iter()
//         .rev()
//         .cloned()
//         .collect::<Vec<u8>>();
//
//     Ok(Request::new(reversed.into()))
// }

