use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use super::parser::{CommandType, Arithmetic};

pub enum Segment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Temp,
    Pointer
}

pub struct Code {
    writer: BufWriter<File>
}

impl Code {
    pub fn new(path: &Path) -> Self {
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        Code { writer }
    } 

    pub fn write_arithmetic(&mut self, command: Arithmetic) {
        let res = Self::arithmetic(command);
        for mut s in res {
            s.push_str("\r\n");
            self.writer.write_all(s.as_bytes()).unwrap();
        }
        self.writer.flush().unwrap();
    }

    fn arithmetic(command: Arithmetic) -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP");
        res.push("M=M-1");
        res.push("@SP");
        res.push("A=M");
        res.push("D=M");
        res.push("@SP");
        res.push("M=M-1");
        res.push("@SP");
        res.push("A=M");
        res.push("D=M+D");
        res.push("M=D");
        res.push("@SP");
        res.push("M=M+1");
        res.iter().map(|s| s.to_string()).collect()
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: Segment, index: i32) {
        match command {
            CommandType::PUSH => {
                let res = Self::push(segment, index);
                for mut s in res {
                    s.push_str("\r\n");
                    self.writer.write_all(s.as_bytes()).unwrap();
                }
            },
            _ => {}
        }
        self.writer.flush().unwrap();
    }

    fn push(segment: Segment, index: i32) -> Vec<String> {
        let mut res = Vec::new();
        res.push(format!("@{}", index));
        res.push("D=A".to_string());
        res.push("@SP".to_string());
        res.push("A=M".to_string());
        res.push("M=D".to_string());
        res.push("@SP".to_string());
        res.push("M=M+1".to_string());
        res
    }
}