use super::color;
use std::time::{Duration};
use std::fmt::Display;
use term::color::{Background, Foreground, Value};

use super::Object;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Clone)]
pub struct Sprite {
    symbol: char,
    foreground: Option<Foreground>,
    background: Option<Background>,
}

impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}", 
            if let Some(fg) = &self.foreground {fg.to_ansi()} else {"".to_string()},
            if let Some(bg) = &self.background {bg.to_ansi()} else {"".to_string()},
            self.symbol,
        )
    }
}

impl Sprite {
    pub fn new(sym: char, bg: Option<Background>, fg: Option<Foreground>) -> Self {
        Self{
            symbol: sym,
            foreground: fg,
            background: bg
        }
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }

    pub fn fg(&self) -> Option<Foreground>{
        self.foreground.clone()
    }

    pub fn bg(&self) -> Option<Background> {
        self.background.clone()
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from(if let Some(fg) = self.fg() { fg.to_ansi()} else {"".to_string()});
        s.push(self.symbol());
        if self.bg().is_some() {
            s.push_str(&self.bg().unwrap().to_ansi());
        }
        return s;
    }
}

pub fn dwarf_object() -> Object {
    Object::new_static('D', None, Some(Foreground::white(false)))
}

pub fn hurt_dwarf_object(millis: u64) -> Object {
    Object::new_dynamic(vec![
            Sprite::new('D', None, Some(Foreground::red(false))),
            Sprite::new('D', None, Some(Foreground::white(false)))
        ], Duration::from_millis(millis)).unwrap()
}