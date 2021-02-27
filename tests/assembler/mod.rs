// use nand2tetris_assember::run;
use nand2tetris::assembler::Assembler;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn compare_two_files(one: &Path, other: &Path) -> bool {
    let f = File::open(one).unwrap();
    let mut one_reader = BufReader::new(f);
    let f = File::open(other).unwrap();
    let mut other_reader = BufReader::new(f);
    let mut one_line = String::new();
    let mut other_line = String::new();
    let mut equal = true;
    loop {
        one_line.clear();
        let one_size = one_reader.read_line(&mut one_line).unwrap();
        other_line.clear();
        let other_size = other_reader.read_line(&mut other_line).unwrap();
        if one_line.trim() != other_line.trim() {
            equal = false;
            println!("{} with size {} and {} with size {} are not equal", one_line, one_size, other_line, other_size);
            break;
        }

        // eof
        if one_size == 0 {
            break;
        }
    }
    equal
}

fn get_filepath(name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("tests/assembler");
    path.push(name);
    path
}

fn test_assembler(filename: &str) {
    let mut assembler = Assembler::new(&get_filepath(filename));
    assembler.run();
    let mut one_path = get_filepath(filename);
    one_path.set_extension("hack");
    let mut other_path = one_path.clone();
    other_path.set_extension("cmp");
    assert!(compare_two_files(&one_path, &other_path));
}

#[test]
fn test_add() {
    test_assembler("Add.asm");
}

#[test]
fn test_max_less_symbol() {
    test_assembler("MaxL.asm");
}

#[test]
fn test_rect_less_symbol() {
    test_assembler("RectL.asm");
}

#[test]
fn test_pong_less_symbol() {
    test_assembler("PongL.asm");
}

#[test]
fn test_max() {
    test_assembler("Max.asm");
}

#[test]
fn test_rect() {
    test_assembler("Rect.asm");
}

#[test]
fn test_pong() {
    test_assembler("Pong.asm");
}
