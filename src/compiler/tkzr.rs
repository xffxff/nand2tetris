use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum KeyWorld {
    Class,
    Method,
    Function,
    Constructor,
    Int, 
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    KeyWorld(KeyWorld),
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

pub struct Tokenizer {
    reader: BufReader<File>,
    pub current_token: String,
    eof: bool,
}

impl Tokenizer {
    pub fn new(path: &Path) -> Self {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        Tokenizer {
            reader,
            current_token: String::new(),
            eof: false,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        !self.eof
    }

    // pub fn advance(&mut self) {
    //     loop {
    //         if !self.has_more_commands() {
    //             break;
    //         }
    //         self.current_token.clear();
    //         let len = self.reader.read_line(&mut self.current_token).unwrap();
    //         if len == 0 {
    //             self.eof = true;
    //         }
    //         let current_token = match self.current_token.find("//") {
    //             Some(size) => &self.current_token[..size],
    //             None => &self.current_token,
    //         };
    //         self.current_token = current_token.trim().to_string();
    //         if self.command_type() != CommandType::WhiteSpace {
    //             break;
    //         }
    //     }
    // }

    // pub fn token_type(&self) -> TokenType {
        
    // }

    fn key_world(&self) -> Option<KeyWorld> {
        if self.current_token == String::from("class") {
            Some(KeyWorld::Class)
        } else if self.current_token == String::from("constructor") {
            Some(KeyWorld::Constructor)
        } else if self.current_token == String::from("function") {
            Some(KeyWorld::Function)
        } else if self.current_token == String::from("method") {
            Some(KeyWorld::Method)
        } else if self.current_token == String::from("field") {
            Some(KeyWorld::Field)
        } else if self.current_token == String::from("static") {
            Some(KeyWorld::Static)
        } else if self.current_token == String::from("var") {
            Some(KeyWorld::Var)
        } else if self.current_token == String::from("int") {
            Some(KeyWorld::Int)
        } else if self.current_token == String::from("char") {
            Some(KeyWorld::Char)
        } else if self.current_token == String::from("boolean") {
            Some(KeyWorld::Boolean)
        } else if self.current_token == String::from("void") {
            Some(KeyWorld::Void)
        } else if self.current_token == String::from("true") {
            Some(KeyWorld::True)
        } else if self.current_token == String::from("false") {
            Some(KeyWorld::False)
        } else if self.current_token == String::from("null") {
            Some(KeyWorld::Null)
        } else if self.current_token == String::from("this") {
            Some(KeyWorld::This)
        } else if self.current_token == String::from("let") {
            Some(KeyWorld::Let)
        } else if self.current_token == String::from("do") {
            Some(KeyWorld::Do)
        } else if self.current_token == String::from("if") {
            Some(KeyWorld::If)
        } else if self.current_token == String::from("else") {
            Some(KeyWorld::Else)
        } else if self.current_token == String::from("while") {
            Some(KeyWorld::While)
        } else if self.current_token == String::from("return") {
            Some(KeyWorld::Return)
        } else {
            None
        }
    }
    
    fn symbol(&self) -> Option<String> {
        let symbols = vec!["{", "}", "(", ")", "[", "]", ".", ",", ";", "+", "-", "*", "/", "&", "|", "<", ">", "=", "~"];
        if symbols.contains(&self.current_token.as_str()) {
            return Some(self.current_token.clone());
        }
        None
    }

    pub fn reset(&mut self) {
        self.reader.seek(SeekFrom::Start(0)).unwrap();
        self.current_token.clear();
        self.eof = false;
    }
}