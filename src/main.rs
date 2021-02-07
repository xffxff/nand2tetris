use nand2tetris_assember::code::Code;
use nand2tetris_assember::parser::{CommandType, Parser};
use nand2tetris_assember::run;

fn main() {
    let filename = "Add.asm".to_string();
    run(filename);
}
