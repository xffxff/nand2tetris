use super::code::Code;
use super::parser::{CommandType, Parser};
use super::table::SymbolTalbe;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct Assembler {
    path: PathBuf,
    symbol_table: SymbolTalbe,
    parser: Parser,
}

impl Assembler {
    pub fn new(path: &Path) -> Self {
        let parser = Parser::new(path);
        let symbol_table = SymbolTalbe::new();
        Assembler {
            path: path.to_path_buf(),
            symbol_table,
            parser,
        }
    }

    pub fn run(&mut self) {
        self.first_pass();
        self.second_pass();
    }

    fn first_pass(&mut self) {
        let mut line_num = 0;
        while self.parser.has_more_commands() {
            self.parser.advance();
            match self.parser.command_type() {
                CommandType::ACommand => line_num += 1,
                CommandType::CCommand => line_num += 1,
                CommandType::LCommand => {
                    let symbol = self.parser.symbol();
                    self.symbol_table.add_entry(&symbol, line_num);
                }
                CommandType::WhiteSpace => {}
            }
        }
    }

    fn second_pass(&mut self) {
        let hack_path = get_hack_path(&self.path);
        let mut hack_file = File::create(hack_path).unwrap();
        self.parser.reset();
        while self.parser.has_more_commands() {
            self.parser.advance();
            if self.parser.command_type() == CommandType::CCommand {
                let mut bits = format!(
                    "111{}{}{}",
                    Code::comp(&self.parser.comp()),
                    Code::dest(&self.parser.dest()),
                    Code::jump(&self.parser.jump())
                );
                bits.push_str("\r\n");
                hack_file.write_all(bits.as_bytes()).unwrap();
            } else if self.parser.command_type() == CommandType::ACommand {
                let symbol = self.parser.symbol();
                let mut bits = Code::symbol(&symbol, &mut self.symbol_table);
                bits.push_str("\r\n");
                hack_file.write_all(bits.as_bytes()).unwrap();
            }
            hack_file.flush().unwrap();
        }
    }
}

fn get_hack_path(path: &Path) -> PathBuf {
    let mut path = path.to_path_buf();
    path.set_extension("hack");
    path
}
