//! Server state manager structs

use std::sync::Arc;

use futures::SinkExt;
use service::WebSocketWriteStream;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

pub mod service;

/// The server's current state, whether driver is distracted and the current data buffer receiving
/// channel
#[derive(Default)]
pub struct ServerState {
    /// Is the driver currently distracted
    pub distracted: bool,
    /// Websocket messaging channel
    pub video_feed: Option<WebSocketWriteStream>,
}

impl ServerState {
    /// Creates a new server state that is thread safe
    pub fn to_async(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }

    /// Returns whether the user is distracted
    pub fn is_distracted(&self) -> bool {
        self.distracted
    }

    /// Toggles the distracted bool
    pub fn toggle_distraction(&mut self) {
        self.distracted = !self.distracted
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
