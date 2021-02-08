use std::env;
use nand2tetris_assember::assembler::Assembler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("too many or too few arguments, only one argument is required");
    }

    let mut assembler = Assembler::new(&args[1]);
    assembler.run();
}
