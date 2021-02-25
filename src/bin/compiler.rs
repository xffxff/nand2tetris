use nand2tetris_assember::compiler::compilation_engine::CompilationEngine;
use std::path::Path;

fn main() {
    let path = Path::new("Square/Square.jack");
    let mut compile_engine = CompilationEngine::new(&path);
    compile_engine.compile_class();
}
