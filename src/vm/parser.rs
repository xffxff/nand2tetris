use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
    WHITESPACE,
}

pub struct Parser {
    reader: BufReader<File>,
    pub current_command: String,
    eof: bool,
}

impl Parser {
    pub fn new(path: &Path) -> Self {
        let f = File::open(path).unwrap();
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
            let current_command = match self.current_command.find("//") {
                Some(size) => &self.current_command[..size],
                None => &self.current_command,
            };
            self.current_command = current_command.trim().to_string();
            if self.command_type() != CommandType::WHITESPACE {
                break;
            }
        }
    }

    pub fn command_type(&self) -> CommandType {
        if self.current_command.is_empty() || self.current_command.starts_with("//") {
            CommandType::WHITESPACE
        } else if self.current_command.starts_with("push") {
            CommandType::PUSH
        } else if self.current_command.starts_with("pop") {
            CommandType::POP
        } else if self.current_command.starts_with("label") {
            CommandType::LABEL
        } else if self.current_command.starts_with("goto") {
            CommandType::GOTO
        } else if self.current_command.starts_with("if-goto") {
            CommandType::IF
        } else if self.current_command.starts_with("function") {
            CommandType::FUNCTION
        } else if self.current_command.starts_with("return") {
            CommandType::RETURN
        } else if self.current_command.starts_with("call") {
            CommandType::CALL
        } else {
            CommandType::ARITHMETIC
        }
    }

    pub fn arg1(&self) -> String {
        let arg1 = match self.command_type() {
            CommandType::ARITHMETIC => &self.current_command,
            _ => {
                let v: Vec<&str> = self.current_command.split(' ').collect();
                v[1]
            }
        };
        arg1.to_string()
    }

    pub fn arg2(&self) -> i32 {
        let arg2 = match self.command_type() {
            CommandType::PUSH | CommandType::POP | CommandType::FUNCTION | CommandType::CALL => {
                let v: Vec<&str> = self.current_command.split(' ').collect();
                v[2]
            }
            _ => panic!("{:?} command does not have arg2", self.command_type()),
        };
        arg2.parse().unwrap()
    }

    pub fn reset(&mut self) {
        self.reader.seek(SeekFrom::Start(0)).unwrap();
        self.current_command.clear();
        self.eof = false;
    }
}
