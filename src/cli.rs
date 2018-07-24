use std::io;
use std::io::BufRead;
use std::sync::mpsc::Sender;

use ::command::Command;


pub(super) struct CLI {
    channel: Sender<Command>,
}


impl CLI {
    pub(super) fn new(channel: Sender<Command>) -> Self {
        CLI {
            channel: channel,
        }
    }

    pub(super) fn run(&self) {
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            match line {
                Ok(line) => if &line == "stop" {
                    break
                } else {
                    self.process_line(line)
                }
                Err(_) => break
            }
        }
    }

    fn process_line(&self, line: String) {
        unimplemented!()
    }
}