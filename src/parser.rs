use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, hint::unreachable_unchecked};

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
    WhiteSpace,
}

pub struct Parser {
    reader: BufReader<File>,
    pub current_command: String,
    eof: bool,
}

impl Parser {
    pub fn new(filename: String) -> Self {
        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        Parser {
            reader,
            current_command: String::new(),
            eof: false,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        !self.eof
    }

    pub fn advance(&mut self) {
        loop {
            if !self.has_more_commands() {
                break;
            }
            self.current_command.clear();
            let len = self.reader.read_line(&mut self.current_command).unwrap();
            if len == 0 {
                self.eof = true;
            }
            self.current_command = self.current_command.trim().to_string();
            if self.command_type() != CommandType::WhiteSpace {
                break;
            }
        }
    }

    pub fn command_type(&self) -> CommandType {
        if self.current_command.len() == 0 || self.current_command.starts_with("//") {
            return CommandType::WhiteSpace;
        }
        if self.current_command.starts_with("@") {
            return CommandType::ACommand;
        }
        return CommandType::CCommand;
    }

    pub fn dest(&self) -> String {
        if self.command_type() != CommandType::CCommand {
            panic!("current command is not a C Command!");
        }
        let dest = match self.current_command.find("=") {
            Some(size) => &self.current_command[..size],
            None => "",
        };

        return dest.to_string();
    }

    pub fn comp(&self) -> String {
        if self.command_type() != CommandType::CCommand {
            panic!("current command is not a C Command!");
        }
        let comp_and_jump = match self.current_command.find("=") {
            Some(size) => &self.current_command[size + 1..],
            None => &self.current_command[..],
        };
        let comp = match comp_and_jump.find(";") {
            Some(size) => &comp_and_jump[..size],
            None => &comp_and_jump[..],
        };
        comp.to_string()
    }

    pub fn jump(&self) -> String {
        if self.command_type() != CommandType::CCommand {
            panic!("current command is not a C Command!");
        }
        let jump = match self.current_command.find(";") {
            Some(size) => &self.current_command[size + 1..],
            None => "",
        };
        jump.to_string()
    }

    pub fn symbol(&self) -> String {
        if self.command_type() != CommandType::ACommand {
            panic!("current command is not a A Command!");
        }
        let symbol = match self.current_command.find("@") {
            Some(size) => &self.current_command[size + 1..],
            None => "",
        };
        let symbol: i16 = symbol.parse().unwrap();
        format!("{:016b}", symbol)
    }
}
