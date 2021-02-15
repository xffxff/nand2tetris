use nand2tetris_assember::vm::parser::{Parser, CommandType};
use nand2tetris_assember::vm::code::{Code, Segment};
use std::path::Path;

fn main() {
    let path = Path::new("BasicTest.vm");
    let mut parser = Parser::new(path);
    let path = Path::new("BasicTest.asm");
    let mut code = Code::new(path);
    while parser.has_more_commands() {
        parser.advance();
        // println!("{} {}", parser.arg1(), parser.arg2());
        match parser.command_type() {
            CommandType::ARITHMETIC => {
                let command = parser.arg1();
                code.write_arithmetic(&command);
            },
            CommandType::PUSH => {
                let segment = parser.arg1();
                code.write_push_pop(CommandType::PUSH, &segment, parser.arg2());
            },
            CommandType::POP => {
                let segment = parser.arg1();
                code.write_push_pop(CommandType::POP, &segment, parser.arg2());
            }
            _ => {}
        }
    }
}