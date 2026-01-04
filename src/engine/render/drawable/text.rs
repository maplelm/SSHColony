use super::Character as Char;
use my_term::color::{Background, Foreground};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct TextSlice<'a> {
    data: &'a [Char],
}

impl<'a> TextSlice<'a> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a> std::fmt::Display for TextSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.data.iter().enumerate() {
            if i == 0 || !c.same_colors(&self.data[i - 1]) {
                if let Err(e) = write!(f, "{c}") {
                    return std::fmt::Result::Err(e);
                }
            } else {
                if let Err(e) = write!(f, "{}", c.as_char()) {
                    return std::fmt::Result::Err(e);
                }
            }
        }
        write!(f, "\x1b[0m")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Text {
    data: Vec<Char>,
}

impl Text {
    pub fn from(s: impl Into<String>, fg: impl Into<u8>, bg: impl Into<u8>) -> Self {
        let mut v = vec![];
        let fg = fg.into();
        let bg = bg.into();
        for c in s.into().chars() {
            v.push(Char::new(c, fg, bg));
        }
        Self { data: v }
    }

    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn push(&mut self, c: Char) {
        self.data.push(c);
    }

    pub fn insert(&mut self, index: usize, c: Char) {
        self.data.insert(index, c);
    }

    pub fn push_str(&mut self, s: &str, fg: u8, bg: u8) {
        for c in s.chars() {
            self.data.push(Char::new(c, fg, bg));
        }
    }

    pub fn insert_str(&mut self, index: usize, s: &str, fg: u8, bg: u8) {
        let mut i = 0;
        for c in s.chars() {
            self.data.insert(index + i, Char::new(c, fg, bg));
            i += 1;
        }
    }

    pub fn push_text(&mut self, text: &Text) {
        for c in text.data.iter() {
            self.data.push(c.clone());
        }
    }

    pub fn insert_text(&mut self, index: usize, text: &Text) {
        for (i, c) in text.data.iter().enumerate() {
            self.data.insert(index + i, c.clone());
        }
    }

    pub fn push_textslice(&mut self, text: &TextSlice) {
        for c in text.data.iter() {
            self.data.push(c.clone());
        }
    }

    pub fn insert_textslice(&mut self, index: usize, text: &TextSlice) {
        for (i, c) in text.data.iter().enumerate() {
            self.data.insert(index + i, c.clone());
        }
    }

    pub fn pop(&mut self) -> Option<Char> {
        self.data.pop()
    }

    pub fn remove(&mut self, index: usize) -> Char {
        self.data.remove(index)
    }

    pub fn join(&mut self, other: Text) {
        for c in other {
            self.push(c);
        }
    }

    pub fn split(mut self, index: usize) -> (Text, Text) {
        let (l, r) = self.data.split_at(index);
        (l.into(), r.into())
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        if self.len() > 0 {
            for c in self.data.iter() {
                output.push(c.as_char());
            }
        }
        output
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> TextSlice {
        TextSlice { data: &self.data }
    }

    pub fn slice(&self, start: usize, end: usize) -> TextSlice {
        let end = if end > self.len() { self.len() } else { end };
        TextSlice {
            data: &self.data[start..end],
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.data.iter().enumerate() {
            if i == 0 || !c.same_colors(&self.data[i - 1]) {
                if let Err(e) = write!(f, "{c}") {
                    return std::fmt::Result::Err(e);
                }
            } else {
                if let Err(e) = write!(f, "{}", c.as_char()) {
                    return std::fmt::Result::Err(e);
                }
            }
        }
        write!(f, "\x1b[0m")
    }
}

impl IntoIterator for Text {
    type Item = Char;
    type IntoIter = std::vec::IntoIter<Char>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a> IntoIterator for &'a Text {
    type Item = &'a Char;
    type IntoIter = std::slice::Iter<'a, Char>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

pub trait PushText {
    fn push_text(&mut self, t: &Text);
}

impl PushText for String {
    fn push_text(&mut self, t: &Text) {
        write!(self, "{t}");
    }
}

#[cfg(test)]
mod test {

    use super::Char;
    use super::Text;

    #[test]
    fn text_basic_test() {
        let text = Text::from("hello", 2, 0);
        println!("{text}")
    }

    #[test]
    fn text_multicolor_test() {
        let mut text = Text::from("Mulicolor Text", 2, 0);
        text.insert(9, Char::new(',', 0, 1));
        //let output = format!("{text}");
        //assert_eq!(output.as_str(), "\x1b[")
        println!("{text}");
    }

    #[test]
    fn text_join_test() {
        let mut a = Text::from("Hello, ", 2, 1);
        let b = Text::from("World!", 1, 2);
        a.join(b);
        println!("{a}");
    }
}
