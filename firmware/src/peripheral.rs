//! The peripherals driver, delivers the "payloads" for certain predictions

use std::time::Duration;

use jetgpio::gpio::pins::OutputPin;

/// Moves certain peripherals when the driver is distracted
pub fn deliver_distracted_payload(relay: &mut OutputPin) {
    for _ in 1..10 {
        println!("Turning relay ON");
        relay.set_high().unwrap();
        std::thread::sleep(Duration::from_millis(50));

        println!("Turning relay OFF");
        relay.set_low().unwrap();
        std::thread::sleep(Duration::from_millis(50));
    }
}
