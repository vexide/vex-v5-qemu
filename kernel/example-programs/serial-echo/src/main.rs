#![no_main]
#![no_std]

use alloc::string::{String, ToString};
use core::{hint::black_box, time::Duration};

use vex_sdk::vexSerialReadChar;
use vexide::{
    core::io::{self, stdin, stdout, Stdin},
    prelude::*,
};

extern crate alloc;

#[vexide::main]
async fn main(peripherals: Peripherals) {
    println!("Hello, world!");
    let mut buf = String::new();
    loop {
        let mut bytes_buffer = [0u8; Stdin::STDIN_BUFFER_SIZE];
        if let Ok(bytes) = stdin().read(&mut bytes_buffer) {
            buf.extend(bytes_buffer.iter().take(bytes).map(|byte| *byte as char));
        }
        while let Some(pair) = buf.split_once('\n') {
            println!("You typed: {}", pair.0);
            buf = pair.1.to_string();
        }
        sleep(Duration::from_millis(250)).await;
    }
}
