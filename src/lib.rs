extern crate vlc;

use std::thread::spawn;
use std::sync::mpsc;

mod command;
mod executor;
mod cli;
mod player;


pub fn run() {
    let (sender, receiver) = mpsc::channel();
    let c = cli::CLI::new(sender);
    spawn(move || c.run());
    let e_handle = spawn(move || {
        let player = player::Player::new().expect("Failed to load player!");
        let mut e = executor::Executor::new(receiver, player);
        e.run()
    });
    e_handle.join();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
