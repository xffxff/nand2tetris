use std::path::Path;
use nand2tetris_assember::compiler::tkzr::Tokenizer;

fn main() {
    let path = Path::new("Square/Square.jack");
    let tkzr = Tokenizer::new(&path);
}