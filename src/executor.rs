use std::sync::mpsc;

use ::command::Command;


pub(super) struct Executor {
    channel: mpsc::Receiver<Command>,
}


impl Executor {
    pub(super) fn new(channel: mpsc::Receiver<Command>) -> Self {
        Self {
            channel: channel,
        }
    }

    pub(super) fn run(&self) {
        loop {
            match self.channel.recv() {
                Ok(command) => self.execute(command),
                Err(_) => return
            }
        }
    }

    fn execute(&self, cmd: Command) {
        match cmd {
        }
    }
}