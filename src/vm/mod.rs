pub mod code;
pub mod parser;

use std::path::{Path, PathBuf};
use code::Code;
use parser::{Parser, CommandType};
use std::fs;
use std::io;
use std::ffi::OsStr;

pub struct VM {
    code: code::Code,
    files: Vec<PathBuf>
}

impl VM {
    pub fn new(path: &Path) -> Self {
        match path.is_file() {
            true => {
                let mut files = Vec::new(); 
                files.push(path.to_path_buf());
                let path = get_asm_path(path);
                let code = Code::new(&path);
                VM { code, files }
            }
            false => {
                let files = fs::read_dir(path).unwrap()
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, io::Error>>().unwrap();
                let files = files.into_iter().filter(|x| x.extension() == Some(OsStr::new("vm"))).collect();
                let path = get_asm_path(path);
                let code = Code::new(&path);
                VM { code, files }                
            }
        }
    }

    pub fn translate(&mut self) {
        self.code.write_init();
        for file in self.files.clone() {
            let filename = file.file_name().unwrap().to_string_lossy().to_string();
            self.code.set_filename(&filename);
            self.translate_one_file(&file)
        }
    }

    fn translate_one_file(&mut self, path: &Path) {
        let mut parser = Parser::new(path);
        while parser.has_more_commands() {
            parser.advance();
            match parser.command_type() {
                CommandType::ARITHMETIC => {
                    let command = parser.arg1();
                    self.code.write_arithmetic(&command);
                }
                CommandType::PUSH => {
                    let segment = parser.arg1();
                    self.code.write_push_pop(CommandType::PUSH, &segment, parser.arg2());
                }
                CommandType::POP => {
                    let segment = parser.arg1();
                    self.code.write_push_pop(CommandType::POP, &segment, parser.arg2());
                }
                CommandType::LABEL => {
                    let label = parser.arg1();
                    self.code.write_label(&label);
                }
                CommandType::IF => {
                    let label = parser.arg1();
                    self.code.write_if(&label);
                }
                CommandType::GOTO => {
                    let label = parser.arg1();
                    self.code.write_goto(&label);
                }
                CommandType::FUNCTION => {
                    self.code.write_function(&parser.arg1(), parser.arg2());
                }
                CommandType::RETURN => {
                    self.code.write_return();
                }
                CommandType::CALL => {
                    self.code.write_call(&parser.arg1(), parser.arg2());
                }
                _ => {}
            }
        }
    }

}

fn get_asm_path(path: &Path) -> PathBuf {
    let mut path = path.to_path_buf();
    path.set_extension("asm");
    path
}
