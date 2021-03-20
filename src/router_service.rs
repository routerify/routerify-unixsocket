use crate::request_meta::RequestMeta;
use hyper::{body::HttpBody, service::Service};
use routerify::{RequestServiceBuilder, Router};
use std::{
    convert::Infallible,
    future::{ready, Ready},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    task::{Context, Poll},
};
use tower::util::{BoxService, ServiceExt};

/// A [`Service`](hyper::service::Service) to process incoming requests.
/// This is adapted from routerify's [`RouterService`](routerify::RouterService) in order to support handling the [`tokio::net::UnixStream`]
/// instance passed by [`hyperlocal`]
///
/// This `RouterService<B, E>` type accepts two type parameters: `B` and `E`.
///
/// * The `B` represents the response body type which will be used by route handlers and the middlewares and this body type must implement
///   the [`HttpBody`](hyper::body::HttpBody) trait. For an instance, `B` could be [`hyper::Body`](hyper::body::Body)
///   type.
/// * The `E` represents any error type which will be used by route handlers and the middlewares. This error type must implement the [`std::error::Error`].
#[derive(Debug)]
pub struct UnixRouterService<B, E>
where
    B: HttpBody + Send + Sync + 'static,
{
    builder: RequestServiceBuilder<B, E>,
}

impl<
        B: HttpBody + Send + Sync + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
    > UnixRouterService<B, E>
{
    /// Creates a new service with the provided router and it's ready to be used with the hyper [`serve`](hyper::server::Builder::.serve)
    /// method.
    pub fn new(router: Router<B, E>) -> routerify::Result<UnixRouterService<B, E>> {
        let builder = RequestServiceBuilder::new(router)?;
        Ok(UnixRouterService { builder })
    }
}

impl<
        B: HttpBody + Send + Sync + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
    > Service<&tokio::net::UnixStream> for UnixRouterService<B, E>
{
    type Response =
        BoxService<hyper::Request<hyper::Body>, hyper::Response<B>, routerify::RouteError>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, conn: &tokio::net::UnixStream) -> Self::Future {
        let loopback = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
        let meta = RequestMeta::new(conn);

        let req_service = self.builder.build(loopback).map_request(
            move |mut req: hyper::Request<hyper::Body>| {
                let ext = req.extensions_mut();
                if ext.get_mut::<RequestMeta>().is_none() {
                    ext.insert(meta.clone());
                }
                req
            },
        );
        ready(Ok(BoxService::new(req_service)))
    }
}
