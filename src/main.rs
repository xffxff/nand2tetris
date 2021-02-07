use nand2tetris_assember::routine::Routine;

fn main() {
    let mut routine = Routine::new("Add.asm".to_string());
    for _ in 0..10 {
        routine.advance();
        print!("{}", routine.current_cmd);
    }
}

