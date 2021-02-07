pub mod code;
pub mod parser;

use code::Code;
use parser::{CommandType, Parser};
use std::fs::File;
use std::io::Write;

pub fn run(filename: &str) {
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
            bits.push_str("\r\n");
            hack_file.write_all(bits.as_bytes()).unwrap();
        } else if parser.command_type() == CommandType::ACommand {
            let mut bits = parser.symbol();
            bits.push_str("\r\n");
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

mod tests {
    use super::run;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    fn compare_two_files(one: &str, other: &str) -> bool {
        let f = File::open(one).unwrap();
        let mut one_reader = BufReader::new(f);
        let f = File::open(other).unwrap();
        let mut other_reader = BufReader::new(f);
        let mut one_line = String::new();
        let mut other_line = String::new();
        let mut equal = true;
        loop {
            one_line.clear();
            let one_size = one_reader.read_line(&mut one_line).unwrap();
            other_line.clear();
            let other_size = other_reader.read_line(&mut other_line).unwrap();
            if one_size != other_size || one_line != other_line {
                equal = false;
                println!("{} and {} are not equal", one_line, other_line);
                break
            }

            // eof
            if one_size == 0 {
                break
            }
        }
        equal
    }

    #[test]
    fn test_add() {
        run("Add.asm");
        assert!(compare_two_files("Add.cmp", "Add.hack"));
    }
}