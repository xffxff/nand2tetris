use nand2tetris_assember::code::Code;
use nand2tetris_assember::parser::{CommandType, Parser};

fn main() {
    let mut parser = Parser::new("Add.asm".to_string());
    for _ in 0..50 {
        parser.advance();
        if parser.command_type() == CommandType::CCommand {
            println!(
                "{} {} {} {}",
                parser.current_command,
                parser.dest(),
                parser.comp(),
                parser.jump(),
            );
        }
    }
}
