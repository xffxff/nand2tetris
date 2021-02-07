pub mod code;
pub mod parser;

use code::Code;
use parser::{CommandType, Parser};

pub fn run(filename: String) {
    let mut parser = Parser::new(filename);
    while parser.has_more_commands() {
        parser.advance();
        if parser.command_type() == CommandType::CCommand {
            let bits = format!(
                "111{}{}{}",
                Code::comp(&parser.comp()),
                Code::dest(&parser.dest()),
                Code::jump(&parser.jump())
            );
            println!("{}", bits);
        } else if parser.command_type() == CommandType::ACommand {
            let bits = parser.symbol();
            println!("{}", bits);
        }
    }
}
