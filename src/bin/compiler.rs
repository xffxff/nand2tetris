use nand2tetris_assember::compiler::Compiler;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("too many or too few arguments, only one argument is required");
    }

    let path = Path::new(&args[1]);
    let compiler = Compiler::new(&path);
    compiler.compile();
}
