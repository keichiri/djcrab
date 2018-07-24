extern crate vlc;

use std::thread::spawn;
use std::sync::mpsc;

mod command;
mod executor;
mod cli;


pub fn run() {
    let (sender, receiver) = mpsc::channel();
    let c = cli::CLI::new(sender);
    let e = executor::Executor::new(receiver);
    spawn(move || c.run());
    let e_handle = spawn(move || e.run());
    e_handle.join();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
