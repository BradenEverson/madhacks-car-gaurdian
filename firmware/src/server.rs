//! Server state manager structs

use std::sync::Arc;

use futures::SinkExt;
use service::WebSocketWriteStream;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

pub mod service;

/// The server's current state and the current data buffer receiving
/// channel
#[derive(Default)]
pub struct ServerState {
    /// Websocket messaging channel
    pub video_feed: Option<WebSocketWriteStream>,
}

impl ServerState {
    /// Creates a new server state that is thread safe
    pub fn to_async(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }

    /// Assigns a websocket write stream to the state
    pub fn assign_websocket(&mut self, video_feed: WebSocketWriteStream) {
        self.video_feed = Some(video_feed);
    }

    /// Sends a data buffer down the websocket (if we have a websocket)
    pub async fn send_buffer(
        &mut self,
        data: &[u8],
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        if let Some(write) = &mut self.video_feed {
            write.send(Message::binary(data)).await
        } else {
            Ok(())
        }
    }
}
