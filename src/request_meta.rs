use std::sync::Arc;

use tokio::net::{
    unix::{SocketAddr, UCred},
    UnixStream,
};

/// RequestMeta holds information about the current request
/// It is injected as an extension within [`UnixRouterService`]. This struct is internal, and accessors are provided on the [`Request`](hyper::Request) object,
/// through the [`UnixRequestExt`] trait.
#[derive(Clone)]
pub(crate) struct RequestMeta {
    peer_addr: Option<Arc<SocketAddr>>,
    cred: Option<UCred>,
}

impl RequestMeta {
    pub(crate) fn new(conn: &UnixStream) -> Self {
        Self {
            peer_addr: conn.peer_addr().ok().map(Arc::new),
            cred: conn.peer_cred().ok(),
        }
    }

    pub fn peer_addr(&self) -> Option<&SocketAddr> {
        self.peer_addr.as_deref()
    }

    pub fn peer_cred(&self) -> Option<&UCred> {
        self.cred.as_ref()
    }
}
