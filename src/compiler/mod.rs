pub mod compilation_engine;
pub mod tkzr;

use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::ffi::OsStr;
use compilation_engine::CompilationEngine;

pub struct Compiler {
    files: Vec<PathBuf>,
}

impl Compiler {
    pub fn new(path: &Path) -> Self {
        if path.is_file() {
            let mut files = Vec::new();
            files.push(path.to_path_buf());
            Compiler { files }
        } else {
            let files = fs::read_dir(path)
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
                .unwrap();
            let files = files
                .into_iter()
                .filter(|x| x.extension() == Some(OsStr::new("jack")))
                .collect();
            Compiler { files }
        }
    }

    pub fn compile(&self) {
        for file in self.files.clone() {
            println!("compiling {:?}", file);
            let mut engine = CompilationEngine::new(&file);
            engine.compile_class();
        }
    }
}