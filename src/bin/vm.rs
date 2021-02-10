use nand2tetris_assember::vm::parser::{Parser, CommandType};
use std::path::Path;

fn main() {
    let path = Path::new("SimpleAdd.vm");
    let mut parser = Parser::new(path);
    while parser.has_more_commands() {
        parser.advance();
        println!("{} {}", parser.arg1(), parser.arg2());
    }
}