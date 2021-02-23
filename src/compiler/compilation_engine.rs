use super::tkzr::{Tokenizer, KeyWorld};
use std::path::Path;
use xml::writer::{EventWriter, EmitterConfig, XmlEvent};

pub struct CompilationEngine {
    tkzr: Tokenizer,
}

impl CompilationEngine {
    pub fn new(path: &Path) -> Self {
        let tkzr = Tokenizer::new(path);
        CompilationEngine {
            tkzr
        }
    }

    pub fn compile_class(&self) {

    }

    fn compile_key_world(key_world: &KeyWorld) {
        let events = Vec::new();
        let key_world_string = format!("{}", key_world);
        let start_event = XmlEvent::start_element(&key_world_string).into();
    }
}