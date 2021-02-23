use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
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
    Symbol(String),
    Identifier(String),
    IntConst(i16),
    StringConst(String),
}

pub struct Tokenizer {
    pub current_token: String,
    tokens: VecDeque<String>,
}

impl Tokenizer {
    pub fn new(path: &Path) -> Self {
        let f = File::open(path).unwrap();
        let mut reader = BufReader::new(f);
        let tokens = Self::get_all_tokens(&mut reader);
        Tokenizer {
            tokens,
            current_token: String::new(),
        }
    }

    pub fn has_more_commands(&self) -> bool {
        !self.tokens.is_empty()
    }

    fn get_all_tokens(reader: &mut BufReader<File>) -> VecDeque<String> {
        let mut commenting = false;
        let mut tokens = VecDeque::new();
        loop {
            let mut new_line = String::new();
            let len = reader.read_line(&mut new_line).unwrap();
            if len == 0 {
                break;
            }
            new_line = new_line.trim().to_string();
            if new_line.starts_with("/*") {
                if !new_line.ends_with("*/") {
                    commenting = true;
                }
                continue;
            }
            if new_line.ends_with("*/") {
                commenting = false;
                continue;
            }
            if commenting {
                continue;
            }
            new_line = match new_line.find("//") {
                Some(size) => new_line[..size].to_string(),
                None => new_line,
            };
            new_line = new_line.trim().to_string();
            if new_line.len() == 0 {
                continue;
            }

            tokens.extend(Self::get_tokens_from_one_line(&new_line));
        }
        tokens
    }

    fn get_tokens_from_one_line(line: &str) -> Vec<String> {
        let mut res = Vec::new();
        let mut last = 0;
        for (index, matched) in line.match_indices(|c: char| !c.is_alphanumeric()) {
            if last != index {
                res.push(&line[last..index]);
            }
            res.push(matched);
            last = index + matched.len();
        }
        if last < line.len() {
            res.push(&line[last..]);
        }
        let res = res
            .iter()
            .map(|x| x.to_string())
            .filter(|x| x.trim().len() != 0)
            .collect::<Vec<_>>();
        res
    }

    pub fn advance(&mut self) {
        self.current_token = self.tokens.pop_front().unwrap();
    }

    pub fn token_type(&self) -> TokenType {
        let key_world = self.key_world();
        if key_world.is_some() {
            return TokenType::KeyWorld(key_world.unwrap());
        }

        let symbol = self.symbol();
        if symbol.is_some() {
            return TokenType::Symbol(symbol.unwrap());
        }

        let int_val = self.int_val();
        if int_val.is_some() {
            return TokenType::IntConst(int_val.unwrap());
        }
        return TokenType::Identifier(self.current_token.clone());
    }

    fn key_world(&self) -> Option<KeyWorld> {
        if self.current_token == "class" {
            Some(KeyWorld::Class)
        } else if self.current_token == "constructor" {
            Some(KeyWorld::Constructor)
        } else if self.current_token == "function" {
            Some(KeyWorld::Function)
        } else if self.current_token == "method" {
            Some(KeyWorld::Method)
        } else if self.current_token == "field" {
            Some(KeyWorld::Field)
        } else if self.current_token == "static" {
            Some(KeyWorld::Static)
        } else if self.current_token == "var" {
            Some(KeyWorld::Var)
        } else if self.current_token == "int" {
            Some(KeyWorld::Int)
        } else if self.current_token == "char" {
            Some(KeyWorld::Char)
        } else if self.current_token == "boolean" {
            Some(KeyWorld::Boolean)
        } else if self.current_token == "void" {
            Some(KeyWorld::Void)
        } else if self.current_token == "true" {
            Some(KeyWorld::True)
        } else if self.current_token == "false" {
            Some(KeyWorld::False)
        } else if self.current_token == "null" {
            Some(KeyWorld::Null)
        } else if self.current_token == "this" {
            Some(KeyWorld::This)
        } else if self.current_token == "let" {
            Some(KeyWorld::Let)
        } else if self.current_token == "do" {
            Some(KeyWorld::Do)
        } else if self.current_token == "if" {
            Some(KeyWorld::If)
        } else if self.current_token == "else" {
            Some(KeyWorld::Else)
        } else if self.current_token == "while" {
            Some(KeyWorld::While)
        } else if self.current_token == "return" {
            Some(KeyWorld::Return)
        } else {
            None
        }
    }

    fn symbol(&self) -> Option<String> {
        let symbols = vec![
            "{", "}", "(", ")", "[", "]", ".", ",", ";", "+", "-", "*", "/", "&", "|", "<", ">",
            "=", "~",
        ];
        if symbols.contains(&self.current_token.as_str()) {
            return Some(self.current_token.clone());
        }
        None
    }

    fn int_val(&self) -> Option<i16> {
        match self.current_token.parse::<i16>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
