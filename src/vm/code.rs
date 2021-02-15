use super::parser::{Arithmetic, CommandType};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

pub enum Segment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Temp,
    Pointer,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Segment::Local => write!(f, "LCL"),
            Segment::Argument => write!(f, "ARG"),
            Segment::This => write!(f, "THIS"),
            Segment::That => write!(f, "THAT"),
            Segment::Constant => write!(f, "CONSTANT"),
            Segment::Static => write!(f, "STATIC"),
            Segment::Temp => write!(f, "TEMP"),
            Segment::Pointer => write!(f, "POINTER"),
        }
    }
}

pub struct Code {
    writer: BufWriter<File>,
    filename: String,
    label_count: i32,
}

impl Code {
    pub fn new(path: &Path) -> Self {
        let filename = path.file_stem().unwrap().to_string_lossy().to_string();
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        Code {
            writer,
            filename,
            label_count: 0,
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        let command = Self::str2arithmetic(command);
        let res = match command {
            Arithmetic::Add => Self::add_sub("+"),
            Arithmetic::Sub => Self::add_sub("-"),
            Arithmetic::Neg => Self::neg(),
            Arithmetic::Eq => self.compare("JEQ"),
            Arithmetic::Lt => self.compare("JLT"),
            Arithmetic::Gt => self.compare("JGT"),
            Arithmetic::And => Self::and_or("&"),
            Arithmetic::Or => Self::and_or("|"),
            Arithmetic::Not => Self::not(),
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
        res.push("M=M-1"); // SP--
        res.push("@SP");
        res.push("A=M");
        res.push("D=M"); // D = *SP
        res.push("@SP");
        res.push("M=M-1"); // SP--
        res.push("@SP");
        res.push("A=M");
        let asm = format!("D=M{}D", cmd);
        res.push(&asm); // D = *SP - D
        res.push("M=D"); // *SP = D
        res.push("@SP");
        res.push("M=M+1"); // SP++
        res.iter().map(|s| s.to_string()).collect()
    }

    fn neg() -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP"); // SP--
        res.push("M=M-1");
        res.push("@SP");
        res.push("A=M");
        res.push("D=M"); // D = *SP
        res.push("M=-D"); // *SP = -D
        res.push("@SP");
        res.push("M=M+1");
        res.iter().map(|s| s.to_string()).collect()
    }

    fn compare(&mut self, cmp: &str) -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP");
        res.push("M=M-1"); // SP--
        res.push("@SP");
        res.push("A=M");
        res.push("D=M"); // D = *SP
        res.push("@SP");
        res.push("M=M-1"); // SP--;

        res.push("@SP");
        res.push("A=M");
        res.push("D=M-D"); // D = *SP - D
        let goto_label = format!("@EQ.{}", self.label_count);
        res.push(&goto_label);
        let cmp_asm = format!("D;{}", cmp);
        res.push(&cmp_asm); // jump EQ
        res.push("@SP");
        res.push("A=M");
        res.push("M=0"); // *SP = 0
        let goto_end = format!("@END.{}", self.label_count);
        res.push(&goto_end);
        res.push("0;JMP");

        let label = format!("(EQ.{})", self.label_count);
        res.push(&label); // *SP == -1
        res.push("@SP");
        res.push("A=M");
        res.push("M=-1");

        let end = format!("(END.{})", self.label_count);
        res.push(&end);
        res.push("@SP");
        res.push("M=M+1");
        self.label_count += 1;
        res.iter().map(|s| s.to_string()).collect()
    }

    fn and_or(cmd: &str) -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP"); // SP--
        res.push("M=M-1");
        res.push("@SP"); // D = *SP
        res.push("A=M");
        res.push("D=M");
        res.push("@SP"); // SP--
        res.push("M=M-1");
        res.push("@SP"); // *SP = *SP & D
        res.push("A=M");
        let asm = format!("M=M{}D", cmd);
        res.push(&asm);
        res.push("@SP");
        res.push("M=M+1");
        res.iter().map(|s| s.to_string()).collect()
    }

    fn not() -> Vec<String> {
        let mut res = Vec::new();
        res.push("@SP"); // SP--
        res.push("M=M-1");
        res.push("@SP"); // *SP = !*SP
        res.push("A=M");
        res.push("M=!M");
        res.push("@SP");
        res.push("M=M+1");
        res.iter().map(|s| s.to_string()).collect()
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: i32) {
        let segment = Self::str2segment(segment);
        let res = match command {
            CommandType::PUSH => self.push(segment, index),
            CommandType::POP => self.pop(segment, index),
            _ => panic!("Invalid command, must be one of PUSH or POP!"),
        };
        for mut s in res {
            s.push_str("\r\n");
            self.writer.write_all(s.as_bytes()).unwrap();
        }
        self.writer.flush().unwrap();
    }

    fn push(&self, segment: Segment, index: i32) -> Vec<String> {
        let mut res = Vec::new();
        match segment {
            Segment::Constant => {
                let index = format!("@{}", index);
                res.push(index);
                res.push("D=A".to_string());
            }
            Segment::Pointer => {
                let segment = match index {
                    0 => Segment::This,
                    1 => Segment::That,
                    _ => panic!("Not a valid index, must be 0 or 1 for Pointer"),
                };
                let index = format!("@{}", segment);
                res.push(index);
                res.push("D=M".to_string());
            }
            Segment::Static => {
                let index = format!("@{}.{}", self.filename, index);
                res.push(index);
                res.push("D=M".to_string());
            }
            Segment::Temp => {
                let index = format!("@{}", index + 5);
                res.push(index);
                res.push("D=M".to_string());
            }
            _ => {
                let segment_start = format!("@{}", segment);
                res.push(segment_start); // addr = segment + i
                res.push("D=M".to_string());
                let index = format!("@{}", index);
                res.push(index);
                res.push("D=D+A".to_string());
                res.push("A=D".to_string());
                res.push("D=M".to_string());
            }
        }
        res.push("@SP".to_string());
        res.push("A=M".to_string());
        res.push("M=D".to_string());

        res.push("@SP".to_string());
        res.push("M=M+1".to_string());
        res
    }

    fn pop(&mut self, segment: Segment, index: i32) -> Vec<String> {
        let mut res = Vec::new();
        match segment {
            Segment::Temp => {
                let index = format!("@{}", index + 5);
                res.push(index); // addr = 5 + i
                res.push("D=A".to_string());
                res.push("@addr".to_string());
                res.push("M=D".to_string());

                res.push("@SP".to_string()); // SP--
                res.push("M=M-1".to_string());

                res.push("@SP".to_string()); // *addr = *SP
                res.push("A=M".to_string());
                res.push("D=M".to_string());
                res.push("@addr".to_string());
                res.push("A=M".to_string());
                res.push("M=D".to_string());
            }
            Segment::Pointer => {
                let segment = match index {
                    0 => Segment::This,
                    1 => Segment::That,
                    _ => panic!("Not a valid index, must be 0 or 1 for Pointer"),
                };
                res.push("@SP".to_string()); // SP--
                res.push("M=M-1".to_string());

                res.push("@SP".to_string()); // THIS = *SP
                res.push("A=M".to_string());
                res.push("D=M".to_string());
                let seg = format!("@{}", segment);
                res.push(seg);
                res.push("M=D".to_string());
            }
            Segment::Static => {
                res.push("@SP".to_string()); // SP--
                res.push("M=M-1".to_string());

                res.push("@SP".to_string()); // *filename.i = *SP
                res.push("A=M".to_string());
                res.push("D=M".to_string());
                let index = format!("@{}.{}", self.filename, index);
                res.push(index);
                res.push("M=D".to_string());
            }
            _ => {
                let segment_start = format!("@{}", segment);
                res.push(segment_start); // addr = (segment + i)
                res.push("D=M".to_string());
                let index = format!("@{}", index);
                res.push(index);
                res.push("D=D+A".to_string());
                res.push("@addr".to_string());
                res.push("M=D".to_string());

                res.push("@SP".to_string()); // SP--
                res.push("M=M-1".to_string());

                res.push("@SP".to_string()); // *addr = *SP
                res.push("A=M".to_string());
                res.push("D=M".to_string());
                res.push("@addr".to_string());
                res.push("A=M".to_string());
                res.push("M=D".to_string());
            }
        };
        res
    }

    fn str2arithmetic(s: &str) -> Arithmetic {
        match s {
            "add" => Arithmetic::Add,
            "sub" => Arithmetic::Sub,
            "neg" => Arithmetic::Neg,
            "eq" => Arithmetic::Eq,
            "gt" => Arithmetic::Gt,
            "lt" => Arithmetic::Lt,
            "and" => Arithmetic::And,
            "or" => Arithmetic::Or,
            "not" => Arithmetic::Not,
            _ => panic!("not a valid arithmetic string"),
        }
    }

    fn str2segment(s: &str) -> Segment {
        match s {
            "local" => Segment::Local,
            "argument" => Segment::Argument,
            "this" => Segment::This,
            "that" => Segment::That,
            "constant" => Segment::Constant,
            "static" => Segment::Static,
            "temp" => Segment::Temp,
            "pointer" => Segment::Pointer,
            _ => panic!("not a valid segment string"),
        }
    }
}
