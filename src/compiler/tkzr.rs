use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum KeyWord {
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

impl fmt::Display for KeyWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KeyWord::Class => write!(f, "class"),
            KeyWord::Method => write!(f, "method"),
            KeyWord::Function => write!(f, "function"),
            KeyWord::Constructor => write!(f, "constructor"),
            KeyWord::Int => write!(f, "int"),
            KeyWord::Boolean => write!(f, "boolean"),
            KeyWord::Char => write!(f, "char"),
            KeyWord::Void => write!(f, "void"),
            KeyWord::Var => write!(f, "var"),
            KeyWord::Static => write!(f, "static"),
            KeyWord::Field => write!(f, "field"),
            KeyWord::Let => write!(f, "let"),
            KeyWord::Do => write!(f, "do"),
            KeyWord::If => write!(f, "if"),
            KeyWord::Else => write!(f, "else"),
            KeyWord::While => write!(f, "while"),
            KeyWord::Return => write!(f, "return"),
            KeyWord::True => write!(f, "true"),
            KeyWord::False => write!(f, "false"),
            KeyWord::Null => write!(f, "null"),
            KeyWord::This => write!(f, "this"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    KeyWord(KeyWord),
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
        let mut current_token = self.tokens.pop_front().unwrap();
        // There is a bug in the logic of processing strings, but I don't
        // want to spend time in this place.
        if current_token == "\"" {
            let mut next_token = self.tokens.pop_front().unwrap();
            while next_token != "\"" {
                current_token.push_str(" ");
                current_token.push_str(&next_token);
                next_token = self.tokens.pop_front().unwrap();
            }
            current_token.push_str(&next_token);
        }
        self.current_token = current_token;
    }

    pub fn next_token(&mut self) -> Option<String> {
        match self.tokens.front() {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }

    pub fn token_type(&self) -> TokenType {
        let key_world = self.key_world();
        if key_world.is_some() {
            return TokenType::KeyWord(key_world.unwrap());
        }

        let symbol = self.symbol();
        if symbol.is_some() {
            return TokenType::Symbol(symbol.unwrap());
        }

        let int_val = self.int_val();
        if int_val.is_some() {
            return TokenType::IntConst(int_val.unwrap());
        }

        let string_val = self.string_val();
        if string_val.is_some() {
            return TokenType::StringConst(string_val.unwrap());
        }
        return TokenType::Identifier(self.current_token.clone());
    }

    fn key_world(&self) -> Option<KeyWord> {
        if self.current_token == "class" {
            Some(KeyWord::Class)
        } else if self.current_token == "constructor" {
            Some(KeyWord::Constructor)
        } else if self.current_token == "function" {
            Some(KeyWord::Function)
        } else if self.current_token == "method" {
            Some(KeyWord::Method)
        } else if self.current_token == "field" {
            Some(KeyWord::Field)
        } else if self.current_token == "static" {
            Some(KeyWord::Static)
        } else if self.current_token == "var" {
            Some(KeyWord::Var)
        } else if self.current_token == "int" {
            Some(KeyWord::Int)
        } else if self.current_token == "char" {
            Some(KeyWord::Char)
        } else if self.current_token == "boolean" {
            Some(KeyWord::Boolean)
        } else if self.current_token == "void" {
            Some(KeyWord::Void)
        } else if self.current_token == "true" {
            Some(KeyWord::True)
        } else if self.current_token == "false" {
            Some(KeyWord::False)
        } else if self.current_token == "null" {
            Some(KeyWord::Null)
        } else if self.current_token == "this" {
            Some(KeyWord::This)
        } else if self.current_token == "let" {
            Some(KeyWord::Let)
        } else if self.current_token == "do" {
            Some(KeyWord::Do)
        } else if self.current_token == "if" {
            Some(KeyWord::If)
        } else if self.current_token == "else" {
            Some(KeyWord::Else)
        } else if self.current_token == "while" {
            Some(KeyWord::While)
        } else if self.current_token == "return" {
            Some(KeyWord::Return)
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
            match self.current_token.as_str() {
                "<" => return Some("&lt;".to_string()),
                ">" => return Some("&gt;".to_string()),
                "\"" => return Some("&quot;".to_string()),
                "&" => return Some("&amp;".to_string()),
                _ => return Some(self.current_token.clone()),
            }
        }
        None
    }

    fn int_val(&self) -> Option<i16> {
        match self.current_token.parse::<i16>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    fn string_val(&self) -> Option<String> {
        if self.current_token.starts_with("\"") {
            let current_token = self.current_token.trim_start_matches("\"");
            let current_token = current_token.trim_end_matches("\"");
            return Some(current_token.to_string());
        }
        None
    }
}
