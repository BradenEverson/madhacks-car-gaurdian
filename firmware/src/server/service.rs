//! Server Service Struct Implementation

use super::ServerState;
use futures::{stream::SplitSink, StreamExt};
use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    upgrade::Upgraded,
    Method, Request, Response, StatusCode,
};
use std::{fs::File, io::Read, sync::Arc};
use std::{future::Future, pin::Pin};
use tokio::sync::RwLock;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

/// A websocket write stream
pub type WebSocketWriteStream =
    SplitSink<WebSocketStream<hyper_util::rt::tokio::TokioIo<Upgraded>>, Message>;

/// A server service implementation responsible for updating the state
#[derive(Clone)]
pub struct ServerService {
    /// The internal state
    pub state: Arc<RwLock<ServerState>>,
}

impl ServerService {
    /// Creates a new server service from a thread safe ServerState
    pub fn new(state: Arc<RwLock<ServerState>>) -> Self {
        Self { state }
    }
}

impl Service<Request<body::Incoming>> for ServerService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, mut req: Request<body::Incoming>) -> Self::Future {
        if hyper_tungstenite::is_upgrade_request(&req) {
            let (response, websocket) =
                hyper_tungstenite::upgrade(&mut req, None).expect("Error upgrading to WebSocket");

            let state = self.state.clone();

            tokio::spawn(async move {
                match websocket.await {
                    Ok(ws) => {
                        let (writer, _) = ws.split();
                        {
                            state.write().await.assign_websocket(writer);
                        }
                    }
                    Err(err) => panic!("Websocket connect error: {err}"),
                }
            });

            Box::pin(async { Ok(response) })
        } else {
            let response = Response::builder().status(StatusCode::OK);

            let res = match *req.method() {
                Method::GET => {
                    let file = match req.uri().path() {
                        "/" => "index.html",
                        _ => "404.html",
                    };

                    let file = format!("frontend/{}", file);
                    let mut page = File::open(file).expect("Failed to open file");
                    let mut buf = vec![];

                    page.read_to_end(&mut buf)
                        .expect("Failed to read to buffer");

                    response.body(Full::new(Bytes::copy_from_slice(&buf)))
                }
                _ => response.body(Full::new(Bytes::copy_from_slice(&[]))),
            };

            Box::pin(async { res })
        }
    }
}
