use nand2tetris_assember::vm::parser::{Parser, CommandType};
use nand2tetris_assember::vm::code::{Code, Segment};
use std::path::Path;

fn main() {
    let path = Path::new("SimpleAdd.vm");
    let mut parser = Parser::new(path);
    let path = Path::new("SimpleAdd.asm");
    let mut code = Code::new(path);
    while parser.has_more_commands() {
        parser.advance();
        // println!("{} {}", parser.arg1(), parser.arg2());
        match parser.command_type() {
            CommandType::ARITHMETIC(command) => {
                code.write_arithmetic(command);
            },
            CommandType::PUSH => {
                code.write_push_pop(CommandType::PUSH, Segment::Constant, parser.arg2());
            },
            _ => {}
        }
    }
}