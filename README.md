# routerify-unixsocket
[![Crates.io](https://img.shields.io/crates/v/routerify-unixsocket.svg)](https://crates.io/crates/routerify-unixsocket)
[![Docs.rs](https://docs.rs/routerify-unixsocket/badge.svg)](https://docs.rs/routerify-unixsocket)
[![CI](https://github.com/mrene/routerify-unixsocket/workflows/Continuous%20Integration/badge.svg)](https://github.com/mrene/routerify-unixsocket/actions)
[![Coverage Status](https://coveralls.io/repos/github/mrene/routerify-unixsocket/badge.svg?branch=master)](https://coveralls.io/github/mrene/routerify-unixsocket?branch=master)

Routerify <> Hyperlocal

## Usage
Serve unix sockets with routerify

Basic usage works by replacing `RouterService` with `UnixRouterService`, which adapts the 
request in order to be compatible with routerify's `RequestService`.

Since routerify requires an IP `SocketAddr`, the loopback address `127.0.0.1` with port 0 is used as a placeholder. 
In order to access the unix socket's peer address and peer credential, the `UnixRequestExt` extension trait adds methods to the request object.

# Example

```rust
use hyper::{Body, Response, Server};
use hyperlocal::UnixServerExt;
use routerify::{Error, Router};
use routerify_unixsocket::{UnixRequestExt, UnixRouterService};
use std::{fs, path::Path};

#[tokio::main]
async fn main() {
    let path = Path::new("/tmp/hyperlocal.sock");
    if path.exists() {
        fs::remove_file(path).unwrap();
    }

    let router: Router<Body, Error> = Router::builder()
        .get("/", |req| async move {
            let s = format!("You are: {:?}", req.unix_peer_cred());
            Ok(Response::new(Body::from(s)))
        })
        .build()
        .unwrap();

    let service = UnixRouterService::new(router).unwrap();
        Server::bind_unix(path)
            .unwrap()
            .serve(service)
            .await
            .unwrap()
}
```


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
