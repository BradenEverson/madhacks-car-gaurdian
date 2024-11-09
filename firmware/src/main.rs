//! The main driver for the Rust firmware, usese V4L to continuously stream video data and perform
//! CNN analysis on it

use std::fs::File;
use std::io::Write;

use v4l::io::traits::CaptureStream;
use v4l::prelude::MmapStream;
use v4l::{buffer::Type, Device};

fn main() {
    let mut dev = Device::new(0).expect("Failed to open camera");
    let mut stream = MmapStream::with_buffers(&mut dev, Type::VideoCapture, 4)
        .expect("Failed to create buffer stream");

    while let Ok((buf, meta)) = stream.next() {
        println!(
            "Buffer size: {}, sequence: {}, timestamp: {}",
            buf.len(),
            meta.sequence,
            meta.timestamp
        );
        let mut file = File::create("frame.jpg").expect("Create new file");
        file.write_all(&buf).expect("Write current buffer to image");
    }
}
