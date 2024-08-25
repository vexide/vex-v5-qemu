use std::time::Duration;

use tokio::process::Command;
use vex_v5_qemu_host::brain::Brain;

#[tokio::main]
async fn main() {
    let mut brain = Brain::new();

    brain.run_program(
        Command::new("qemu-system-arm"),
        "../kernel/target/armv7a-none-eabi/debug/kernel".into(),
        "/home/tropical/Documents/GitHub/vexide/target/armv7a-vex-v5/debug/examples/basic.bin".into(),
    ).await.unwrap();

    let mut peripherals = brain.peripherals.take().unwrap();

    peripherals.battery.set_voltage(100).await.unwrap();

    loop {
        peripherals.battery.set_voltage(peripherals.battery.voltage() - 1).await.unwrap();
        tokio::time::sleep(Duration::from_millis(150)).await;
    }
}
