use super::tkzr::{self, KeyWorld, Tokenizer, TokenType};
use std::path::Path;
use xml::writer::{EventWriter, EmitterConfig, XmlEvent};
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

pub struct CompilationEngine {
    tkzr: Tokenizer,
    writer: EventWriter<File>
}

impl CompilationEngine {
    pub fn new(path: &Path) -> Self {
        let tkzr = Tokenizer::new(path);
        let mut output_path = path.to_path_buf();
        output_path.set_extension("xml");
        let file = File::create(&output_path).unwrap();
        let writer = EmitterConfig::new().perform_indent(true).create_writer(file);
        CompilationEngine {
            tkzr,
            writer 
        }
    }

    pub fn compile_class(&mut self) {
        while self.tkzr.has_more_commands() {
            self.tkzr.advance();
            match self.tkzr.token_type() {
                TokenType::KeyWorld(key_world) => {
                    let key_world = format!("{}", key_world);
                    let events = Self::compile_key_world(&key_world);
                    self.write_events(events);
                }, 
                TokenType::Symbol(symbol) => {
                    let events = Self::compile_symbol(&symbol);
                    self.write_events(events);
                },
                TokenType::IntConst(val) => {
                    let val = val.to_string();
                    let events = Self::compile_int(&val);
                    self.write_events(events);
                },
                TokenType::StringConst(val) => {
                    let events = Self::compile_string(&val);
                    self.write_events(events);

                }
                _ => {}
            };
        }
    }

    fn write_events(&mut self, events: Vec<XmlEvent>) {
        for event in events {
            self.writer.write(event).unwrap();
        }
    }

    fn compile_key_world(key_world: &str) -> Vec<XmlEvent> {
        let mut events = Vec::new();
        let start_event: XmlEvent = XmlEvent::start_element("keyworld").into();
        events.push(start_event);
        let body: XmlEvent = XmlEvent::characters(key_world).into();
        events.push(body);
        let end_event: XmlEvent = XmlEvent::end_element().into();
        events.push(end_event);
        events
    }

    fn compile_symbol(symbol: &str) -> Vec<XmlEvent> {
        let mut events = Vec::new();
        let start_event: XmlEvent = XmlEvent::start_element("symbol").into();
        events.push(start_event);
        let body: XmlEvent = XmlEvent::characters(symbol).into();
        events.push(body);
        let end_event: XmlEvent = XmlEvent::end_element().into();
        events.push(end_event);
        events
    }

    fn compile_int(val: &str) -> Vec<XmlEvent> {
        let mut events = Vec::new();
        let start_event: XmlEvent = XmlEvent::start_element("integerConstant").into();
        events.push(start_event);
        let body: XmlEvent = XmlEvent::characters(val).into();
        events.push(body);
        let end_event: XmlEvent = XmlEvent::end_element().into();
        events.push(end_event);
        events
    }

    fn compile_string(val: &str) -> Vec<XmlEvent> {
        let mut events = Vec::new();
        let start_event: XmlEvent = XmlEvent::start_element("stringConstant").into();
        events.push(start_event);
        let body: XmlEvent = XmlEvent::characters(val).into();
        events.push(body);
        let end_event: XmlEvent = XmlEvent::end_element().into();
        events.push(end_event);
        events
    }
}