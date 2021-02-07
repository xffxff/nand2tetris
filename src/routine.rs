use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Routine {
    reader: BufReader<File>,
    pub current_cmd: String,
}

impl Routine {
    pub fn new(filename: String) -> Self {
        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        Routine {
            reader,
            current_cmd: String::new(),
        }
    }

    pub fn advance(&mut self) {
        self.current_cmd.clear();
        let len = self.reader.read_line(&mut self.current_cmd).unwrap();
    }
}