//! Routerify <> Hyperlocal
//! Serve unix sockets with routerify
//!
//! Basic usage works by replacing [`RouterService`](routerify::RouterService) with [`UnixRouterService`], which adapts the
//! request in order to be compatible with [`RequestService`](routerify::RequestService).
//!
//! Since routerify requires an IP [`SocketAddr`](std::net::SocketAddr), the loopback address `127.0.0.1` with port 0 is used as a placeholder.
//! In order to access the unix socket's peer address and peer credential, the [`UnixRequestExt`] extension traits adds methods to the request object.
//!
//! # Example
//! ```no_run
//! use hyper::{Body, Response, Server};
//! use hyperlocal::UnixServerExt;
//! use routerify::{Error, Router};
//! use routerify_unixsocket::{UnixRequestExt, UnixRouterService};
//! use std::{fs, path::Path};
//!
//! #[tokio::main]
//! async fn main() {
//!     let path = Path::new("/tmp/hyperlocal.sock");
//!     if path.exists() {
//!         fs::remove_file(path).unwrap();
//!     }
//!
//!     let router: Router<Body, Error> = Router::builder()
//!         .get("/", |req| async move {
//!             let s = format!("You are: {:?}", req.unix_peer_cred());
//!             Ok(Response::new(Body::from(s)))
//!         })
//!         .build()
//!         .unwrap();
//!
//!     let service = UnixRouterService::new(router).unwrap();
//!     Server::bind_unix(path)
//!         .unwrap()
//!         .serve(service)
//!         .await
//!         .unwrap()
//! }
//! ```

mod router_service;
pub use router_service::UnixRouterService;

mod ext;
pub use ext::UnixRequestExt;

mod request_meta;

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::poll_fn;
    use hyper::{Body, Method, Request, Response};
    use routerify::{Error, Router};
    use std::task::Poll;
    use tokio::net::UnixStream;
    use tower::Service;

    #[tokio::test]
    pub async fn unixsocket() {
        const RESPONSE_TEXT: &str = "Hello world!";

        let mut router_service = {
            let router: Router<Body, Error> = Router::builder()
                .get("/", |_req| async move {
                    Ok(Response::new(Body::from(RESPONSE_TEXT)))
                })
                .build()
                .unwrap();

            UnixRouterService::new(router).unwrap()
        };

        poll_fn(|ctx| -> Poll<_> { router_service.poll_ready(ctx) })
            .await
            .expect("router service is not ready");

        let (_, server) = UnixStream::pair().unwrap();

        let mut service = router_service.call(&server).await.unwrap();
        poll_fn(|ctx| -> Poll<_> { service.poll_ready(ctx) })
            .await
            .expect("request service is not ready");

        let req = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(hyper::Body::empty())
            .unwrap();

        let resp: Response<hyper::body::Body> = service.call(req).await.unwrap();
        let body = resp.into_body();
        let body = String::from_utf8(hyper::body::to_bytes(body).await.unwrap().to_vec()).unwrap();
        assert_eq!(RESPONSE_TEXT, body);
    }
}
