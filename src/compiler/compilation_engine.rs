use super::tkzr::{TokenType, Tokenizer, KeyWorld};
use std::{convert::TryInto, fs::File};
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
        let start_event: XmlEvent = XmlEvent::start_element("tokens").into();
        self.writer.write(start_event).unwrap();
        while self.tkzr.has_more_commands() {
            self.tkzr.advance();
            self.compile_current_token();
        }
        let end_event: XmlEvent = XmlEvent::end_element().into();
        self.writer.write(end_event).unwrap();
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
        self.compile_key_world(self.tkzr.token_type());
        self.tkzr.advance();
        loop {
            self.compile_current_token();
            if let TokenType::Symbol(s) = self.tkzr.token_type() {
                if s == ";" {
                    break;
                }
            }
            self.tkzr.advance();
        }
        self.write_end_event();
    }

    fn compile_subroutine_dec(&mut self) {
        self.write_start_event("subroutineDec");
        loop {

        }
    }

    fn compile_current_token(&mut self) {
        match self.tkzr.token_type() {
            TokenType::KeyWorld(key_world) => {
                if key_world == KeyWorld::Static {
                    self.compile_class_var_dec();
                } else {
                    self.compile_key_world(self.tkzr.token_type());
                }
            }
            TokenType::Symbol(_) => {
                self.compile_symbol(self.tkzr.token_type());
            }
            TokenType::IntConst(_) => {
                self.compile_int(self.tkzr.token_type());
            }
            TokenType::StringConst(_) => {
                self.compile_string(self.tkzr.token_type());
            }
            TokenType::Identifier(_) => {
                self.compile_identifier(self.tkzr.token_type());
            }
        };
}

    fn compile_key_world(&mut self, token: TokenType) {
        if let TokenType::KeyWorld(key_world) = token {
            self.write_start_event("keyword");
            let key_world = format!("{}", key_world);
            self.write_characters(&key_world);
            self.write_end_event();
        } else {
            panic!("{:?} is not a KeyWorld", token);
        }
    }

    fn compile_symbol(&mut self, token: TokenType) {
        if let TokenType::Symbol(symbol) = token {
            self.write_start_event("symbol");
            self.write_characters(&symbol);
            self.write_end_event();
        } else {
            panic!("{:?} is not a Symbol", token);
        }
    }

    fn compile_int(&mut self, token: TokenType) {
        if let TokenType::IntConst(val) = token {
            self.write_start_event("integerConstant");
            self.write_characters(&val.to_string());
            self.write_end_event();
        } else {
            panic!("{:?} is not a IntConst", token);
        }
    }

    fn compile_string(&mut self, token: TokenType) {
        if let TokenType::StringConst(val) = token {
            self.write_start_event("stringConstant");
            self.write_characters(&val);
            self.write_end_event();
        } else {
            panic!("{:?} is not a StringConst");
        }
    }

    fn compile_identifier(&mut self, token: TokenType) {
        if let TokenType::Identifier(identifier) = token {
            self.write_start_event("identifier");
            self.write_characters(&identifier);
            self.write_end_event();
        } else {
            panic!("{:?} is not a Identifier");
        }
    }
}
