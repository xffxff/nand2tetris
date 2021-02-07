pub mod code;
pub mod parser;

use code::Code;
use parser::{CommandType, Parser};
use std::fs::File;
use std::io::Write;

pub fn run(filename: String) {
    let hack_filename = get_hack_filename(&filename);
    let mut hack_file = File::create(hack_filename).unwrap();
    let mut parser = Parser::new(filename);
    while parser.has_more_commands() {
        parser.advance();
        if parser.command_type() == CommandType::CCommand {
            let mut bits = format!(
                "111{}{}{}",
                Code::comp(&parser.comp()),
                Code::dest(&parser.dest()),
                Code::jump(&parser.jump())
            );
            println!("{}", bits);
            bits.push_str("\n");
            hack_file.write_all(bits.as_bytes()).unwrap();
        } else if parser.command_type() == CommandType::ACommand {
            let mut bits = parser.symbol();
            println!("{}", bits);
            bits.push_str("\n");
            hack_file.write_all(bits.as_bytes()).unwrap();
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
