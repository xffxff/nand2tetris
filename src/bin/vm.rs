use nand2tetris_assember::vm::VM;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("too many or too few arguments, only one argument is required");
    }

    let path = Path::new(&args[1]);
    let mut vm = VM::new(&path);
    vm.translate();
}
