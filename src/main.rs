#![feature(alloc_system, global_allocator, allocator_api)]

extern crate alloc_system;

use alloc_system::System;

#[global_allocator]
static A: System = System;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    println!("Hello, world!");

    let devs = std::fs::read_dir("/dev")?;
    for dev in devs {
        let dev = dev?.path();
        if dev.strip_prefix("/dev")?.to_str().unwrap().starts_with("sd") {
            println!("{}", dev.display());
        }
    }

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
            },
        }
    }
}
