extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use xml::reader::{EventReader, XmlEvent};
use nand2tetris_assember::compiler::compilation_engine::CompilationEngine;

fn read_into_vec(path: &Path) -> Vec<String> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let parser = EventReader::new(reader);
    let mut res = Vec::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                res.push(name.to_string());
            }
            Ok(XmlEvent::EndElement { name }) => {
                res.push(name.to_string());
            }
            Ok(XmlEvent::Characters(s)) => {
                res.push(s.to_string());
            }
            _ => {}
        }
    }
    res
}

fn compare_two_files(one: &Path, other: &Path) -> bool {
    let one_res = read_into_vec(one);
    let other_res = read_into_vec(other);
    let mut equal = true;
    for (one, other) in one_res.iter().zip(other_res.iter()) {
        let one = one.trim();
        let other = other.trim();
        if one != other {
            eprintln!("{} - {}", one, other);
            equal = false;
            break;
        }
    }
    equal
}

#[test]
fn test_square_main() {
    let path = Path::new("tests/compiler/Square/Main.jack");
    let mut compiler = CompilationEngine::new(path);
    compiler.compile_class();
    let one = Path::new("tests/compiler/Square/MainT.xml");
    let other = Path::new("tests/compiler/Square/Main.xml");
    assert!(compare_two_files(one, other));
}

#[test]
fn test_square_square() {
    let path = Path::new("tests/compiler/Square/Square.jack");
    let mut compiler = CompilationEngine::new(path);
    compiler.compile_class();
    let one = Path::new("tests/compiler/Square/SquareT.xml");
    let other = Path::new("tests/compiler/Square/Square.xml");
    assert!(compare_two_files(one, other));
}

#[test]
fn test_square_square_game() {
    let path = Path::new("tests/compiler/Square/SquareGame.jack");
    let mut compiler = CompilationEngine::new(path);
    compiler.compile_class();
    let one = Path::new("tests/compiler/Square/SquareGameT.xml");
    let other = Path::new("tests/compiler/Square/SquareGame.xml");
    assert!(compare_two_files(one, other));
}

#[test]
fn test_exp_less_square_main() {
    let path = Path::new("tests/compiler/ExpressionLessSquare/Main.jack");
    let mut compiler = CompilationEngine::new(path);
    compiler.compile_class();
    let one = Path::new("tests/compiler/ExpressionLessSquare/MainT.xml");
    let other = Path::new("tests/compiler/ExpressionLessSquare/Main.xml");
    assert!(compare_two_files(one, other));
}

#[test]
fn test_exp_less_square() {
    let path = Path::new("tests/compiler/ExpressionLessSquare/Square.jack");
    let mut compiler = CompilationEngine::new(path);
    compiler.compile_class();
    let one = Path::new("tests/compiler/ExpressionLessSquare/SquareT.xml");
    let other = Path::new("tests/compiler/ExpressionLessSquare/Square.xml");
    assert!(compare_two_files(one, other));
}

#[test]
fn test_exp_less_square_game() {
    let path = Path::new("tests/compiler/ExpressionLessSquare/SquareGame.jack");
    let mut compiler = CompilationEngine::new(path);
    compiler.compile_class();
    let one = Path::new("tests/compiler/ExpressionLessSquare/SquareGameT.xml");
    let other = Path::new("tests/compiler/ExpressionLessSquare/SquareGame.xml");
    assert!(compare_two_files(one, other));
}