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
    writer: BufWriter<File>,
    lable_count: i32
}

impl Code {
    pub fn new(path: &Path) -> Self {
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        Code { writer, lable_count: 0 }
    } 

    pub fn write_arithmetic(&mut self, command: Arithmetic) {
        let res = match command {
            Arithmetic::Add => Self::add_sub("+"),
            Arithmetic::Sub => Self::add_sub("-"),
            Arithmetic::Neg => Self::neg(),
            Arithmetic::Eq => self.compare("JEQ"),
            Arithmetic::Lt => self.compare("JLT"),
            Arithmetic::Gt => self.compare("JGT"),
            Arithmetic::And => Self::and_or("&"),
            Arithmetic::Or => Self::and_or("|"),
            Arithmetic::Not => Self::not()
        };
        for mut s in res {
            s.push_str("\r\n");
            self.writer.write_all(s.as_bytes()).unwrap();
        }
        self.writer.flush().unwrap();
    }

    fn add_sub(cmd: &str) -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP");
        res.push("M=M-1");    // SP--
        res.push("@SP");
        res.push("A=M");
        res.push("D=M");      // D = *SP
        res.push("@SP");
        res.push("M=M-1");    // SP--
        res.push("@SP");
        res.push("A=M");
        let asm = format!("D=M{}D", cmd);
        res.push(&asm);       // D = *SP - D
        res.push("M=D");      // *SP = D
        res.push("@SP");
        res.push("M=M+1");    // SP++
        res.iter().map(|s| s.to_string()).collect()
    }

    fn neg() -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP");      // SP--
        res.push("M=M-1");
        res.push("@SP");
        res.push("A=M");
        res.push("D=M");      // D = *SP
        res.push("M=-D");     // *SP = -D
        res.push("@SP");
        res.push("M=M+1");
        res.iter().map(|s| s.to_string()).collect()
    }

    fn compare(&mut self, cmp: &str) -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP");
        res.push("M=M-1");    // SP--
        res.push("@SP");
        res.push("A=M");
        res.push("D=M");      // D = *SP
        res.push("@SP");
        res.push("M=M-1");    // SP--;

        res.push("@SP");
        res.push("A=M");
        res.push("D=M-D");    // D = *SP - D
        let label_value = format!("@EQ.{}", self.lable_count);
        res.push(&label_value);
        let cmp_asm = format!("D;{}", cmp);
        res.push(&cmp_asm);    // jump EQ
        res.push("@SP");
        res.push("A=M");
        res.push("M=0");      // *SP = 0
        res.push("@SP");
        res.push("M=M-1");

        let label = format!("(EQ.{})", self.lable_count);
        res.push(&label);     // *SP == -1 
        res.push("@SP");
        res.push("A=M");
        res.push("M=-1");     
        res.push("@SP");
        res.push("M=M-1");
        self.lable_count += 1;
        res.iter().map(|s| s.to_string()).collect()
    }

    fn and_or(cmd: &str) -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP");     // SP--
        res.push("M=M-1");
        res.push("@SP");     // D = *SP
        res.push("A=M");
        res.push("D=M");
        res.push("@SP");     // SP--
        res.push("M=M-1");
        res.push("@SP");     // *SP = *SP & D
        res.push("A=M");
        let asm = format!("M=M{}D", cmd);
        res.push(&asm);
        res.push("@SP");
        res.push("M=M+1");
        res.iter().map(|s| s.to_string()).collect()
    }

    fn not() -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP");     // SP--
        res.push("M=M-1");
        res.push("@SP");     // *SP = !*SP
        res.push("A=M");
        res.push("M=!M");
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