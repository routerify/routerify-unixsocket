use crate::request_meta::RequestMeta;
use hyper::Request;
use tokio::net::unix::{SocketAddr, UCred};

/// A extension trait which extends the [`hyper::Request`](https://docs.rs/hyper/0.14.4/hyper/struct.Request.html) type with methods related to requests served through a unix socket.
pub trait UnixRequestExt {
    /// Returns the the incoming request's socket's peer credential
    ///
    /// # Examples
    ///
    /// ```
    /// use hyper::{Body, Response};
    /// use routerify::{RouteParams, Router};
    /// use routerify_unixsocket::UnixRequestExt;
    /// # use std::convert::Infallible;
    ///
    /// # fn run() -> Router<Body, Infallible> {
    /// let router = Router::builder()
    ///     .get("/", |req| async move {
    ///         let peer_addr = req.unix_peer_addr().expect("did not have any peer address");
    ///
    ///         Ok(Response::new(Body::from(format!(
    ///             "Peer address: {:?}",
    ///             peer_addr
    ///         ))))
    ///     })
    ///     .build()
    ///     .unwrap();
    /// # router
    /// # }
    /// # run();
    /// ```
    fn unix_peer_addr(&self) -> Option<&SocketAddr>;

    /// Returns the the incoming request's socket's peer credential
    ///
    /// # Examples
    ///
    /// ```
    /// use hyper::{Body, Response};
    /// use routerify::{RouteParams, Router};
    /// use routerify_unixsocket::UnixRequestExt;
    /// # use std::convert::Infallible;
    ///
    /// # fn run() -> Router<Body, Infallible> {
    /// let router = Router::builder()
    ///     .get("/whoami", |req| async move {
    ///         let peer_addr = req
    ///             .unix_peer_cred()
    ///             .expect("did not have peer credential information");
    ///
    ///         Ok(Response::new(Body::from(format!(
    ///             "uid={} gid={}",
    ///             peer_addr.uid(),
    ///             peer_addr.gid()
    ///         ))))
    ///     })
    ///     .build()
    ///     .unwrap();
    /// # router
    /// # }
    /// # run();
    /// ```
    fn unix_peer_cred(&self) -> Option<&UCred>;
}

impl UnixRequestExt for Request<hyper::Body> {
    fn unix_peer_addr(&self) -> Option<&SocketAddr> {
        self.extensions()
            .get::<RequestMeta>()
            .and_then(|x| x.peer_addr())
    }

    fn unix_peer_cred(&self) -> Option<&UCred> {
        self.extensions()
            .get::<RequestMeta>()
            .and_then(|x| x.peer_cred())
    }
}
