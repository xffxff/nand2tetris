use super::tkzr::{TokenType, Tokenizer};
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
        let start_event: XmlEvent = XmlEvent::start_element("tokens").into();
        self.writer.write(start_event).unwrap();
        while self.tkzr.has_more_commands() {
            self.tkzr.advance();
            match self.tkzr.token_type() {
                TokenType::KeyWorld(key_world) => {
                    let key_world = format!("{}", key_world);
                    self.compile_key_world(&key_world);
                }
                TokenType::Symbol(symbol) => {
                    self.compile_symbol(&symbol);
                }
                TokenType::IntConst(val) => {
                    let val = val.to_string();
                    self.compile_int(&val);
                }
                TokenType::StringConst(val) => {
                    self.compile_string(&val);
                }
                TokenType::Identifier(identifier) => {
                    self.compile_identifier(&identifier);
                }
            };
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

    fn compile_key_world(&mut self, key_world: &str) {
        self.write_start_event("keyword");
        self.write_characters(key_world);
        self.write_end_event();
    }

    fn compile_symbol(&mut self, symbol: &str) {
        self.write_start_event("symbol");
        self.write_characters(symbol);
        self.write_end_event();
    }

    fn compile_int(&mut self, val: &str) {
        self.write_start_event("integerConstant");
        self.write_characters(val);
        self.write_end_event();
    }

    fn compile_string(&mut self, val: &str) {
        self.write_start_event("stringConstant");
        self.write_characters(val);
        self.write_end_event();
    }

    fn compile_identifier(&mut self, identifier: &str) {
        self.write_start_event("identifier");
        self.write_characters(identifier);
        self.write_end_event();
    }
}
