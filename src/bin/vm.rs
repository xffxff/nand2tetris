use nand2tetris_assember::vm::code::Code;
use nand2tetris_assember::vm::parser::{CommandType, Parser};
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("too many or too few arguments, only one argument is required");
    }

    let path = Path::new(&args[1]);
    let mut parser = Parser::new(path);

    let path = get_asm_path(path);
    let mut code = Code::new(&path);
    while parser.has_more_commands() {
        parser.advance();
        match parser.command_type() {
            CommandType::ARITHMETIC => {
                let command = parser.arg1();
                code.write_arithmetic(&command);
            }
            CommandType::PUSH => {
                let segment = parser.arg1();
                code.write_push_pop(CommandType::PUSH, &segment, parser.arg2());
            }
            CommandType::POP => {
                let segment = parser.arg1();
                code.write_push_pop(CommandType::POP, &segment, parser.arg2());
            }
            CommandType::LABEL => {
                let label = parser.arg1();
                code.write_label(&label);
            }
            CommandType::IF => {
                let label = parser.arg1();
                code.write_if(&label);
            }
            _ => {}
        }
    }
}

fn get_asm_path(path: &Path) -> PathBuf {
    let mut path = path.to_path_buf();
    path.set_extension("asm");
    path
}
