// use nand2tetris_assember::run;
use nand2tetris_assember::assembler::Assembler;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn compare_two_files(one: &str, other: &str) -> bool {
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
        if one_size != other_size || one_line != other_line {
            equal = false;
            println!("{} and {} are not equal", one_line, other_line);
            break;
        }

        // eof
        if one_size == 0 {
            break;
        }
    }
    equal
}

#[test]
fn test_add() {
    let mut assembler = Assembler::new("tests/Add.asm");
    assembler.run();
    assert!(compare_two_files("tests/Add.cmp", "tests/Add.hack"));
}

#[test]
fn test_max_less_symbol() {
    let mut assembler = Assembler::new("tests/MaxL.asm");
    assembler.run();
    assert!(compare_two_files("tests/MaxL.cmp", "tests/MaxL.hack"));
}

#[test]
fn test_rect_less_symbol() {
    let mut assembler = Assembler::new("tests/RectL.asm");
    assembler.run();
    assert!(compare_two_files("tests/RectL.cmp", "tests/RectL.hack"));
}

#[test]
fn test_pong_less_symbol() {
    let mut assembler = Assembler::new("tests/PongL.asm");
    assembler.run();
    assert!(compare_two_files("tests/PongL.cmp", "tests/PongL.hack"));
}

#[test]
fn test_max() {
    let mut assembler = Assembler::new("tests/Max.asm");
    assembler.run();
    assert!(compare_two_files("tests/Max.cmp", "tests/Max.hack"));
}

#[test]
fn test_rect() {
    let mut assembler = Assembler::new("tests/Rect.asm");
    assembler.run();
    assert!(compare_two_files("tests/Rect.cmp", "tests/Rect.hack"));
}

#[test]
fn test_pong() {
    let mut assembler = Assembler::new("tests/Pong.asm");
    assembler.run();
    assert!(compare_two_files("tests/Pong.cmp", "tests/Pong.hack"));
}
