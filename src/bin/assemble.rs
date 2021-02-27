use nand2tetris::assembler::Assembler;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("too many or too few arguments, only one argument is required");
    }

    let path = Path::new(&args[1]);
    let mut assembler = Assembler::new(path);
    assembler.run();
}
