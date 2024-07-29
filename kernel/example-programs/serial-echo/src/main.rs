#![no_main]
#![no_std]

use core::time::Duration;

use vexide::{
    core::io::{stdin, stdout},
    prelude::*,
};

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let mut stdout = stdout();
    let mut stdin = stdin().bytes();
    loop {
        if let Some(byte) = stdin.next() {
            let byte = byte.unwrap();
            stdout.write_all(&[byte]).unwrap();
            println!("Got byte");
        } else {
            println!("No byte available");
        }
        sleep(Duration::from_millis(250)).await;
    }
}
