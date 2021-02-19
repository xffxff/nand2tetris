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
    call_count: i32,
}

impl Code {
    pub fn new(path: &Path) -> Self {
        // let filename = path.file_stem().unwrap().to_string_lossy().to_string();
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        Code {
            writer,
            filename: String::new(),
            label_count: 0,
            call_count: 0,
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

    pub fn write_label(&mut self, label: &str) {
        let label = format!("({})\r\n", label);
        self.writer.write_all(label.as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn write_if(&mut self, label: &str) {
        let mut res = Vec::new();
        res.push("@SP"); // SP--
        res.push("M=M-1");
        res.push("@SP"); // if *SP != 0; JUMP
        res.push("A=M");
        res.push("D=M");
        let label = format!("@{}", label);
        res.push(&label);
        res.push("D;JNE");
        
        for s in res {
            let s = format!("{}\r\n", s);
            self.writer.write_all(s.as_bytes()).unwrap();
        }
        self.writer.flush().unwrap();
    }

    pub fn write_goto(&mut self, label: &str) {
        let label = format!("@{}\r\n", label);
        self.writer.write_all(label.as_bytes()).unwrap();
        self.writer.write_all("0;JMP\r\n".as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn write_function(&mut self, function_name: &str, num_vars: i32) {
        let mut res = Vec::new();
        let label = format!("({})", function_name);
        res.push(label);
        for _ in 0..num_vars {
            res.extend(self.push(Segment::Constant, 0));
        }
        for mut s in res {
            s.push_str("\r\n");
            self.writer.write_all(s.as_bytes()).unwrap();
        }
        self.writer.flush().unwrap();
    }

    pub fn write_return(&mut self) {
        let mut res = Vec::new();
        res.push("@LCL".to_string()); // end_frame = LCL
        res.push("D=M".to_string());
        res.push("@end_frame".to_string());
        res.push("M=D".to_string());
        res.push("@5".to_string());
        res.push("A=D-A".to_string()); // ret_addr = *(end_frame - 5)
        res.push("D=M".to_string());
        res.push("@ret_addr".to_string());
        res.push("M=D".to_string());
        res.extend(self.pop(Segment::Argument, 0)); // *ARG = pop()
        res.push("@ARG".to_string()); // SP = ARG + 1
        res.push("D=M".to_string());
        res.push("@SP".to_string());
        res.push("M=D+1".to_string());
        res.push("@end_frame".to_string()); // THAT = *(end_frame - 1);
        res.push("A=M-1".to_string());
        res.push("D=M".to_string());
        res.push("@THAT".to_string());
        res.push("M=D".to_string());
        res.push("@2".to_string()); // THIS = *(end_frame - 2);
        res.push("D=A".to_string());
        res.push("@end_frame".to_string());
        res.push("A=M-D".to_string());
        res.push("D=M".to_string());
        res.push("@THIS".to_string());
        res.push("M=D".to_string());
        res.push("@3".to_string()); // ARG = *(end_frame - 3);
        res.push("D=A".to_string());
        res.push("@end_frame".to_string());
        res.push("A=M-D".to_string());
        res.push("D=M".to_string());
        res.push("@ARG".to_string());
        res.push("M=D".to_string());
        res.push("@4".to_string()); // LCL = *(end_frame - 4);
        res.push("D=A".to_string());
        res.push("@end_frame".to_string());
        res.push("A=M-D".to_string());
        res.push("D=M".to_string());
        res.push("@LCL".to_string());
        res.push("M=D".to_string());
        res.push("@ret_addr".to_string());
        res.push("A=M".to_string());
        res.push("0;JMP".to_string());
        for mut s in res {
            s.push_str("\r\n");
            self.writer.write_all(s.as_bytes()).unwrap();
        }
        self.writer.flush().unwrap();
    }

    pub fn write_call(&mut self, function_name: &str, num_args: i32) {
        let res = self.call(function_name, num_args);
        for mut s in res {
            s.push_str("\r\n");
            self.writer.write_all(s.as_bytes()).unwrap();
        }
        self.writer.flush().unwrap();
    }

    fn call(&mut self, function_name: &str, num_args: i32) -> Vec<String> {
        let mut res = Vec::new();
        let ret_addr_label = format!("{}$ret.{}", function_name, self.call_count);
        self.call_count += 1;
        res.push(format!("@{}", ret_addr_label)); // push retAddrLabel
        res.push("D=A".to_string());
        res.push("@SP".to_string());
        res.push("A=M".to_string());
        res.push("M=D".to_string());
        res.push("@SP".to_string());
        res.push("M=M+1".to_string());
        res.extend(Self::push_segment(Segment::Local)); // push LCL
        res.extend(Self::push_segment(Segment::Argument)); // push ARG
        res.extend(Self::push_segment(Segment::This)); // push THIS
        res.extend(Self::push_segment(Segment::That)); // push THAT
        res.push("@5".to_string()); // ARG = SP - 5 - num_args
        res.push("D=A".to_string());
        res.push("@SP".to_string());
        res.push("D=M-D".to_string());
        res.push(format!("@{}", num_args));
        res.push("D=D-A".to_string());
        res.push("@ARG".to_string());
        res.push("M=D".to_string());
        res.push("@SP".to_string()); // LCL = SP
        res.push("D=M".to_string());
        res.push("@LCL".to_string());
        res.push("M=D".to_string());
        res.push(format!("@{}", function_name)); // goto function_name
        res.push("0;JMP".to_string());
        res.push(format!("({})", ret_addr_label)); 
        res
    }

    fn push_segment(segment: Segment) -> Vec<String> {
        let mut res = Vec::new();
        res.push(format!("@{}", segment));
        res.push("D=M".to_string());
        res.push("@SP".to_string());
        res.push("A=M".to_string());
        res.push("M=D".to_string());
        res.push("@SP".to_string());
        res.push("M=M+1".to_string());
        res
    }

    pub fn write_init(&mut self) {
        let mut res = Vec::new();
        res.push("@256".to_string());
        res.push("D=A".to_string());
        res.push("@SP".to_string());
        res.push("M=D".to_string());
        res.extend(self.call("Sys.init", 0));

        for mut s in res {
            s.push_str("\r\n");
            self.writer.write(s.as_bytes()).unwrap();
        }
        self.writer.flush().unwrap();
    }

    pub fn set_filename(&mut self, filename: &str) {
        self.filename = filename.to_string();
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
            _ => panic!("{} is not a valid arithmetic string", s),
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
