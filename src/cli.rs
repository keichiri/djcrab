use std::io;
use std::io::{Write, BufRead};
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

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
        let stdout = io::stdout();

        loop {
            match stdout.lock().write_all(">> ".as_bytes()) {
                Ok(_) => stdout.lock().flush(),
                Err(_) => break
            };

            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(_) => {
                    line = line.trim().to_owned();
                    if &line == "stop" {
                        break
                    } else if line.trim() == "" {
                        continue
                    } else {
                        self.process_line(line).map(|c| self.channel.send(c));
                    }
                },
                Err(_) => break
            }
        }
    }

    fn process_line(&self, line: String) -> Option<Command> {
        let elements: Vec<String> = line.split(" ").map(|s| s.to_owned()).collect();
        match elements[0].as_str() {
            "pause" => Some(Command::Pause),
            "unpause" => Some(Command::Unpause),
            "list" => self.process_list(&elements[1..]),
            "play" => self.process_play(&elements[1..]),
            _ => unimplemented!()
        }
    }

    fn process_list(&self, elements: &[String]) -> Option<Command> {
        unimplemented!()
    }

    fn process_play(&self, elements: &[String]) -> Option<Command> {
        let path = &elements[0];
        Some(Command::PlayPath(path.to_owned()))
    }
}