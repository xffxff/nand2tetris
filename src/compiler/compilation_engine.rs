use super::tkzr::{TokenType, Tokenizer, KeyWorld};
use std::fs::File;
use std::path::Path;
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

pub struct CompilationEngine {
    tkzr: Tokenizer,
    writer: EventWriter<File>,
}

impl CompilationEngine {
    pub fn new(path: &Path) -> Self {
        let tkzr = Tokenizer::new(path);
        let mut output_path = path.to_path_buf();
        output_path.set_extension("xml");
        let file = File::create(&output_path).unwrap();
        let mut config = EmitterConfig::new();
        config.perform_escaping = false;
        let writer = config
            .write_document_declaration(false)
            .perform_indent(true)
            .create_writer(file);
        CompilationEngine { tkzr, writer }
    }

    pub fn compile_class(&mut self) {
        self.write_start_event("class");
        self.tkzr.advance();
        while self.tkzr.has_more_commands() {
            self.compile_current_token();
        }
        self.compile_current_token();
        self.write_end_event();
    }

    fn write_start_event(&mut self, name: &str) {
        let event: XmlEvent = XmlEvent::start_element(name).into();
        self.writer.write(event).unwrap();
    }

    fn write_characters(&mut self, s: &str) {
        let event: XmlEvent = XmlEvent::characters(s);
        self.writer.write(event).unwrap();
    }

    fn write_end_event(&mut self) {
        let event: XmlEvent = XmlEvent::end_element().into();
        self.writer.write(event).unwrap();
    }

    fn compile_class_var_dec(&mut self) {
        self.write_start_event("classVarDec");
        self.compile_key_world();
        loop {
            if let TokenType::Symbol(s) = self.tkzr.token_type() {
                if s == ";" {
                    break;
                }
            }
            self.compile_current_token();
        }
        self.compile_symbol();
        self.write_end_event();
    }

    fn compile_subroutine_dec(&mut self) {
        self.write_start_event("subroutineDec");
        self.compile_key_world();
        // self.compile_key_world();
        self.compile_current_token();
        self.compile_identifier();
        self.compile_parameter_list();
        self.compile_subroutine_body();
        self.write_end_event();
    }

    fn compile_parameter_list(&mut self) {
        self.compile_symbol();
        self.write_start_event("parameterList");
        loop {
            if let TokenType::Symbol(symbol) = self.tkzr.token_type() {
                if symbol == ")" {
                    break;
                }
            }
            self.compile_current_token();
        }
        self.write_end_event();
        self.compile_symbol();
    }

    fn compile_subroutine_body(&mut self) {
        self.write_start_event("subroutineBody");
        self.compile_symbol();
        let mut open_bracket_num = 1;
        loop {
            if open_bracket_num == 0 {
                break;
            }
            match self.tkzr.token_type() {
                TokenType::Symbol(symbol) => {
                    if symbol == "{" {
                        open_bracket_num += 1;
                    } 
                    if symbol == "}" {
                        open_bracket_num -= 1;
                    }
                    // if open_bracket_num == 0 {
                    //     self.write_end_event();
                    // }
                    self.compile_symbol();
                }, 
                TokenType::KeyWorld(key_world) => {
                    match key_world {
                        KeyWorld::Var => self.compile_var_dec(),
                        KeyWorld::Let | KeyWorld::Do | KeyWorld::Return | KeyWorld::If => {
                            self.compile_statements(); 
                        },
                        _ => self.compile_current_token(),
                    }
                }
                _ => self.compile_current_token()
            }
        }
        self.write_end_event();
    }

    fn compile_var_dec(&mut self) {
        self.write_start_event("varDec");
        self.compile_key_world();
        loop {
            if let TokenType::Symbol(s) = self.tkzr.token_type() {
                if s == ";" {
                    break;
                }
            }
            self.compile_current_token();
        }
        self.compile_symbol();
        self.write_end_event();
    }

    fn compile_statements(&mut self) {
        self.write_start_event("statements");
        loop {
            match self.tkzr.token_type() {
                TokenType::KeyWorld(key_world) => {
                    match key_world {
                        KeyWorld::If => self.compile_if(),
                        KeyWorld::Let => self.compile_let(),
                        KeyWorld::Do => self.compile_do(),
                        KeyWorld::Return => self.compile_return(),
                        KeyWorld::While => self.compile_while(),
                        // KeyWorld::Else => self.compile_else(),
                        _ => break
                    }
                },
                // TokenType::Symbol(symbol) => {
                //     if symbol == "}" {
                //         if self.tkzr.next_token() == Some("else".to_string()) {
                //             self.compile_symbol();
                //         } else {
                //             break
                //         }
                //     } else {
                //         break;
                //     }
                // }
                _ => break
            } 
        }
        self.write_end_event();
    }

    fn compile_let(&mut self) {
        self.write_start_event("letStatement");
        self.compile_key_world();
        if self.tkzr.next_token() == Some("[".to_string()) {
            self.compile_array();
        } else {
            self.compile_identifier();
        }
        self.compile_symbol();
        self.compile_expression();
        self.compile_symbol();
        self.write_end_event();
    }

    fn compile_do(&mut self) {
        self.write_start_event("doStatement");
        self.compile_key_world();
        self.compile_subroutine_call();
        self.compile_symbol();
        self.write_end_event();
    }

    fn compile_return(&mut self) {
        self.write_start_event("returnStatement");
        self.compile_key_world();
        if let TokenType::KeyWorld(_) = self.tkzr.token_type() {
            self.compile_expression();
        }
        self.compile_symbol();
        self.write_end_event();
    }

    fn compile_while(&mut self) {
        self.write_start_event("whileStatement");
        self.compile_key_world();
        self.compile_symbol();
        self.compile_expression();
        self.compile_symbol();
        self.compile_symbol();
        self.compile_statements();
        self.compile_symbol();
        self.write_end_event();
    }

    fn compile_if(&mut self) {
        self.write_start_event("ifStatement");
        self.compile_key_world();
        self.compile_symbol();
        self.compile_expression();
        self.compile_symbol();
        self.compile_symbol();
        self.compile_statements();
        if let TokenType::Symbol(symbol) = self.tkzr.token_type() {
            if symbol == "}" {
                if self.tkzr.next_token() == Some("else".to_string()) {
                    self.compile_symbol();
                    self.compile_else();
                } else {
                    self.compile_symbol();
                } 
            } 
        } 
        self.write_end_event();
    }

    fn compile_else(&mut self) {
        self.compile_key_world();
        self.compile_symbol();
        self.compile_statements();
        self.compile_symbol();
    }

    fn compile_expression(&mut self) {
        self.write_start_event("expression");
        self.compile_term();
        if let TokenType::Symbol(symbol) = self.tkzr.token_type() {
            if symbol == "*" || symbol == "/" || symbol == "|" || symbol == "+" || symbol == "&lt;"
                || symbol == "&amp;" || symbol == "&gt;" || symbol == "-" || symbol == "=" {
                self.compile_symbol();
                self.compile_term();
            }
        }
        self.write_end_event();
    }

    fn compile_term(&mut self) {
        self.write_start_event("term");
        match self.tkzr.token_type() {
            TokenType::Identifier(_) => {
                if self.tkzr.next_token() == Some(".".to_string()) {
                    self.compile_subroutine_call();
                } else if self.tkzr.next_token() == Some("[".to_string()) {
                    self.compile_array();
                } else {
                    self.compile_current_token();
                }
            },
            TokenType::Symbol(symbol) => {
                self.compile_symbol();
                if symbol == "(" {
                    self.compile_expression();
                    self.compile_symbol();
                } else {
                    self.compile_term();
                }
            },
            _ => self.compile_current_token(),
        }
        self.write_end_event();
    }

    fn compile_expression_list(&mut self) {
        self.compile_symbol();
        self.write_start_event("expressionList");
        loop {
            if let TokenType::Symbol(symbol) = self.tkzr.token_type() {
                if symbol == ")" {
                    break;
                }
                if symbol == "," {
                    self.compile_symbol();
                }
            }
            self.compile_expression();
            // self.compile_current_token();
        }
        self.write_end_event();
        self.compile_symbol();
    }

    fn compile_subroutine_call(&mut self) {
        self.compile_identifier();
        if self.tkzr.current_token == "." {
            self.compile_symbol();
            self.compile_identifier();
        }
        self.compile_expression_list();
    }

    fn compile_array(&mut self) {
        self.compile_identifier();
        self.compile_symbol();
        self.compile_expression();
        self.compile_symbol();
    }

    fn compile_current_token(&mut self) {
        match self.tkzr.token_type() {
            TokenType::KeyWorld(key_world) => {
                match key_world {
                    KeyWorld::Static | KeyWorld::Field => {
                        self.compile_class_var_dec();
                    },
                    KeyWorld::Function | KeyWorld::Constructor | KeyWorld::Method => {
                        self.compile_subroutine_dec();
                    },
                    _ => {
                        self.compile_key_world();
                    }
                }
            }
            TokenType::Symbol(_) => {
                self.compile_symbol();
            }
            TokenType::IntConst(_) => {
                self.compile_int();
            }
            TokenType::StringConst(_) => {
                self.compile_string();
            }
            TokenType::Identifier(_) => {
                self.compile_identifier();
            }
        };
}

    fn compile_key_world(&mut self) {
        if let TokenType::KeyWorld(key_world) = self.tkzr.token_type() {
            self.write_start_event("keyword");
            let key_world = format!("{}", key_world);
            self.write_characters(&key_world);
            self.write_end_event();
            if self.tkzr.has_more_commands() {
                self.tkzr.advance();
            }
        } else {
            panic!("{:?} is not a KeyWorld", self.tkzr.token_type());
        }
    }

    fn compile_symbol(&mut self) {
        if let TokenType::Symbol(symbol) = self.tkzr.token_type() {
            self.write_start_event("symbol");
            self.write_characters(&symbol);
            self.write_end_event();
            if self.tkzr.has_more_commands() {
                self.tkzr.advance();
            }
        } else {
            panic!("{:?} is not a Symbol", self.tkzr.token_type());
        }
    }

    fn compile_int(&mut self) {
        if let TokenType::IntConst(val) = self.tkzr.token_type() {
            self.write_start_event("integerConstant");
            self.write_characters(&val.to_string());
            self.write_end_event();
            if self.tkzr.has_more_commands() {
                self.tkzr.advance();
            }
        } else {
            panic!("{:?} is not a IntConst", self.tkzr.token_type());
        }
    }

    fn compile_string(&mut self) {
        if let TokenType::StringConst(val) = self.tkzr.token_type() {
            self.write_start_event("stringConstant");
            self.write_characters(&val);
            self.write_end_event();
            if self.tkzr.has_more_commands() {
                self.tkzr.advance();
            }
        } else {
            panic!("{:?} is not a StringConst", self.tkzr.token_type());
        }
    }

    fn compile_identifier(&mut self) {
        if let TokenType::Identifier(identifier) = self.tkzr.token_type() {
            self.write_start_event("identifier");
            self.write_characters(&identifier);
            self.write_end_event();
            if self.tkzr.has_more_commands() {
                self.tkzr.advance();
            }
        } else {
            panic!("{:?} is not a Identifier", self.tkzr.token_type());
        }
    }
}
