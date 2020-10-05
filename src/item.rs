use same_file::is_same_file;
use std::fs;
use std::path::Path;
use termion::{color, style};

pub struct Item {
    name: String,
    source: String,
    target: String,
    linked: bool,
    selected: bool,
    source_existence: bool,
    target_existence: bool,
}

impl Item {
    pub fn new(name: &str, source: String, target: String) -> Self {
        let source = shellexpand::tilde(&source).to_string();
        let target = shellexpand::tilde(&target).to_string();
        let source_existence = Path::new(&source).exists();
        let target_existence = Path::new(&target).exists();
        println!("{}, {}", source_existence, target_existence);
        let mut linked = false;
        if source_existence && target_existence {
            linked = is_same_file(&source, &target).unwrap();
        }

        Self {
            name: String::from(name),
            source,
            target,
            linked,
            selected: false,
            source_existence,
            target_existence,
        }
    }
    pub fn render(&self) {
        println!(
            "{selected}{linked} {name}: source: {source_existence}{source}, target: {target_existence}{target}{reset}\r",
            selected = if self.selected {
                format!("{}{}", color::Bg(color::Green), color::Fg(color::White))
            } else {
                "".to_string()
            },
            linked = if self.linked { "LINKED" } else { "UNLINKED" },
            name = self.name,
            source_existence = if self.source_existence {
                "🟢"
            } else {
                "🔴"
            },
            source = self.source,
            target_existence = if self.target_existence {
                if self.linked {
                    "🟢"
                } else {
                    "🟡"
                }
            } else {
                "🔴"
            },
            target = self.target,
            reset = style::Reset,
        );
    }
    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn deselect(&mut self) {
        self.selected = false;
    }
    pub fn link(&mut self) {
        if !self.linked && !self.target_existence {
            fs::hard_link(&self.source, &self.target).unwrap();
        }
        self.update();
    }
    pub fn unlink(&mut self) {
        fs::remove_file(&self.target).unwrap();
        self.update();
    }
    fn update(&mut self) {
        self.source_existence = Path::new(&self.source).exists();
        self.target_existence = Path::new(&self.target).exists();
        if self.source_existence && self.target_existence {
            self.linked = is_same_file(&self.source, &self.target).unwrap();
        } else {
            self.linked = false;
        }
    }
}
