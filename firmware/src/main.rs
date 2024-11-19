//! The main driver for the Rust firmware, usese V4L to continuously stream video data and perform
//! CNN analysis on it

use std::fs::File;
use std::io::Write;

use firmware::server::service::ServerService;
use firmware::server::ServerState;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;

use tokio::net::TcpListener;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use v4l::io::traits::CaptureStream;
use v4l::prelude::MmapStream;
use v4l::{buffer::Type, Device};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();

    println!(
        "Listening on http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    let (sender, mut receiver): (UnboundedSender<Vec<u8>>, _) = unbounded_channel();
    let state = ServerState::default().to_async();
    let service = ServerService::new(state.clone());

    tokio::spawn(async move {
        let dev = Device::new(0).expect("Failed to open camera");
        let mut stream = MmapStream::with_buffers(&dev, Type::VideoCapture, 4)
            .expect("Failed to create buffer stream");

        while let Ok((buf, _)) = stream.next() {
            sender.send(buf.to_vec()).expect("Failed to send");
            let mut file = File::create("frame.jpg").expect("Create new file");
            file.write_all(buf).expect("Write current buffer to image");
        }
    });

    tokio::spawn(async move {
        loop {
            let (socket, _) = listener
                .accept()
                .await
                .expect("Error accepting incoming connection");

            let io = TokioIo::new(socket);
            let service = service.clone();

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .serve_connection(io, service)
                    .with_upgrades()
                    .await
                {
                    eprintln!("Error serving connection: {}", e);
                }
            });
        }
    });

    while let Some(buf) = receiver.recv().await {
        {
            state
                .write()
                .await
                .send_buffer(&buf)
                .await
                .expect("Failed to send data buffer");
        }
    }
}
