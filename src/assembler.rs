use crate::code::Code;
use crate::parser::{CommandType, Parser};
use crate::table::SymbolTalbe;
use std::fs::File;
use std::io::prelude::*;

pub struct Assembler {
    filename: String,
    symbol_table: SymbolTalbe,
    parser: Parser,
}

impl Assembler {
    pub fn new(filename: &str) -> Self {
        let parser = Parser::new(filename);
        let symbol_table = SymbolTalbe::new();
        Assembler {
            filename: filename.to_string(),
            symbol_table,
            parser,
        }
    }

    pub fn run(&mut self) {
        self.second_pass();
    }

    fn second_pass(&mut self) {
        let hack_filename = get_hack_filename(&self.filename);
        let mut hack_file = File::create(hack_filename).unwrap();
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
                let mut bits = Code::symbol(&symbol, &self.symbol_table);
                bits.push_str("\r\n");
                hack_file.write_all(bits.as_bytes()).unwrap();
            }
            hack_file.flush().unwrap();
        }
    }
}

fn get_hack_filename(filename: &str) -> String {
    let filename = match filename.find(".") {
        Some(size) => &filename[..size],
        None => {
            panic!("{} not a valid assembly file", filename);
        }
    };
    format!("{}.hack", filename)
}
