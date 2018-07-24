extern crate djcrab;


use std::thread;


fn main() {
    println!("Starting...");
    djcrab::run();
    println!("Stopped");
}