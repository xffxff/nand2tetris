use nand2tetris_assember::compiler::tkzr::Tokenizer;
use std::path::Path;

fn main() {
    let path = Path::new("Square/Square.jack");
    let mut tkzr = Tokenizer::new(&path);
    while tkzr.has_more_commands() {
        tkzr.advance();
        println!("{} {:?}", tkzr.current_token, tkzr.token_type());
    }
}
