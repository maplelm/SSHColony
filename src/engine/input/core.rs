use chrono::Local;
use std::io::Write;

use super::consts::*;

pub type InputBuffer = [u8; MAX_INPUT_LEN];

pub enum KeyEvent {
    Char(char),    // Any Printable Values
    Control(char), // Contrl + Printable Value
    Alt(char),     // Alt + printable Value
    Enter,
    Tab,
    Backspace,
    Escape,
    Up,
    Down,
    Left,
    Right,
    F(u8), //F1-12
}

pub enum OtherEvent {
    EnterFocus,
    LeaveFocus,
    ScreenSizeChange { width: u32, height: u32 },
    Unknown(String),
}

type MouseButtonValue = u8;
const X10_LEFT_CLICK: MouseButtonValue = 1;
const X10_RIGHT_CLICK: MouseButtonValue = 2;
const X10_MIDDLE_CLICK: MouseButtonValue = 3;

const SGR_LEFT_CLICK: MouseButtonValue = 6;
const SGR_RIGHT_CLICK: MouseButtonValue = 7;
const SGR_MIDDLE_CLICK: MouseButtonValue = 8;

pub enum MouseEvent {
    Pressed(MouseData),
    Release(MouseData),
    Move(MouseData),
}

pub struct MouseData {
    button: MouseButton,
    pos: MousePos,
}
pub struct MousePos {
    x: u16,
    y: u16,
}
#[derive(PartialEq, Eq)]
pub enum MouseButton {
    None,
    Left,
    Right,
    Middle,
    Fourth,
    Fith,
}

pub enum Modifier {
    Shift,
    Control,
    Alt,
}

pub fn poll_event(mut seq: &[u8]) -> Option<Event> {
    match seq {
        b"\x1b[A" => Some(Event::Keyboard(KeyEvent::Up)),
        b"\x1b[B" => Some(Event::Keyboard(KeyEvent::Down)),
        b"\x1b[C" => Some(Event::Keyboard(KeyEvent::Right)),
        b"\x1b[D" => Some(Event::Keyboard(KeyEvent::Left)),
        b"\x1bOP" | b"\x1b[11~" | b"\x1b[[A" | b"\x1b[M" => Some(Event::Keyboard(KeyEvent::F(1))),
        b"\x1bOQ" | b"\x1b[12~" | b"\x1b[[B" | b"\x1b[N" => Some(Event::Keyboard(KeyEvent::F(2))),
        b"\x1bOR" | b"\x1b[13~" | b"\x1b[[C" | b"\x1b[O" => Some(Event::Keyboard(KeyEvent::F(3))),
        b"\x1bOS" | b"\x1b[14~" | b"\x1b[[D" | b"\x1b[P" => Some(Event::Keyboard(KeyEvent::F(4))),
        b"\x1b[15~" | b"\x1b[[E" | b"\x1b[Q" => Some(Event::Keyboard(KeyEvent::F(5))),
        b"\x1b[17~" | b"\x1b[R" => Some(Event::Keyboard(KeyEvent::F(6))),
        b"\x1b[18~" | b"\x1b[S" => Some(Event::Keyboard(KeyEvent::F(7))),
        b"\x1b[19~" | b"\x1b[T" => Some(Event::Keyboard(KeyEvent::F(8))),
        b"\x1b[20~" | b"\x1b[U" => Some(Event::Keyboard(KeyEvent::F(9))),
        b"\x1b[21~" | b"\x1b[V" => Some(Event::Keyboard(KeyEvent::F(10))),
        b"\x1b[23~" | b"\x1b[W" => Some(Event::Keyboard(KeyEvent::F(11))),
        b"\x1b[24~" | b"\x1b[X" => Some(Event::Keyboard(KeyEvent::F(12))),
        [b] if b.is_ascii_graphic() || *b == b' ' => {
            Some(Event::Keyboard(KeyEvent::Char(*b as char)))
        }
        b"\n" => Some(Event::Keyboard(KeyEvent::Enter)),
        b"\x1b" => Some(Event::Keyboard(KeyEvent::Escape)),
        [8] | [127] => Some(Event::Keyboard(KeyEvent::Backspace)),
        b"\t" => Some(Event::Keyboard(KeyEvent::Tab)),
        seq if seq.starts_with(b"\x1b[M") || seq.starts_with(b"\x1b[<") => {
            Some(Event::Mouse(parse_mouse_event(seq)?))
        }
        seq if seq.starts_with(b"\x1b[8;") => parse_terminal_size_change(seq),
        _ => None,
    }
}

fn parse_terminal_size_change(seq: &[u8]) -> Option<Event> {
    let str_val = String::from_utf8_lossy(seq);
    let str_val = str_val.strip_prefix("\x1b[8;").unwrap();
    let mut str_val = str_val.strip_suffix('t').unwrap();
    let vals: Vec<&str> = str_val.split(';').collect();
    let x: u32 = vals[1].parse().unwrap();
    let y: u32 = vals[0].parse().unwrap();
    return Some(Event::Other(OtherEvent::ScreenSizeChange {
        width: x,
        height: y,
    }));
}

fn parse_mouse_event(seq: &[u8]) -> Option<MouseEvent> {
    match seq {
        // SGR Mode
        seq if seq.starts_with(b"\x1b[<") => {
            let seq = seq.strip_prefix(b"\x1b[<").unwrap();
            let s = std::str::from_utf8(seq).ok().unwrap();
            let last_char = s.chars().last().unwrap();

            if last_char != 'm' && last_char != 'M' {
                return None; // invalid data form
            }
            let s = &s[0..s.len() - 1];
            let mut split = s.split(';');
            let cb = split.next()?.parse::<u8>().ok()?;
            let cx = split.next()?.parse::<u16>().ok()?;
            let cy = split.next()?.parse::<u16>().ok()?;

            //let btn = cb & 0b11;
            let btn: MouseButton;
            match cb & 0b11 {
                0 => btn = MouseButton::Left,
                1 => btn = MouseButton::Right,
                2 => btn = MouseButton::Middle,
                _ => btn = MouseButton::None,
            }
            let is_motion = cb & 0b0010000 != 0;

            match last_char {
                'M' => {
                    if is_motion {
                        Some(MouseEvent::Move(MouseData {
                            button: btn,
                            pos: MousePos { x: cx, y: cy },
                        }))
                    } else {
                        Some(MouseEvent::Pressed(MouseData {
                            button: btn,
                            pos: MousePos { x: cx, y: cy },
                        }))
                    }
                }
                'm' => Some(MouseEvent::Release(MouseData {
                    button: btn,
                    pos: MousePos { x: cx, y: cy },
                })),
                _ => None,
            }
        }
        // X10 Mode
        seq if seq.starts_with(b"\x1b[M") && seq.len() == 6 => {
            let cb = seq[3].wrapping_sub(32);
            let cx = seq[4].wrapping_sub(32) as u16;
            let cy = seq[5].wrapping_sub(32) as u16;

            let button_bytes = cb & 0b11;
            let is_pressed = 0b1000 == 0;
            let is_motion = cb & 0b0010000 != 0;

            let button: MouseButton;
            match button_bytes {
                0 => button = MouseButton::Left,
                1 => button = MouseButton::Middle,
                2 => button = MouseButton::Right,
                _ => button = MouseButton::None,
            }

            if is_motion {
                return Some(MouseEvent::Move(MouseData {
                    button: button,
                    pos: MousePos { x: cx, y: cy },
                }));
            } else if is_pressed {
                return Some(MouseEvent::Pressed(MouseData {
                    button: button,
                    pos: MousePos { x: cx, y: cy },
                }));
            } else if !is_pressed && !is_motion && button == MouseButton::None {
                return Some(MouseEvent::Release(MouseData {
                    button: MouseButton::None,
                    pos: MousePos { x: cx, y: cy },
                }));
            } else {
                return None;
            }
        }
        _ => None,
    }
}

pub enum Event {
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    Other(OtherEvent),
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Keyboard(kb_event) => match kb_event {
                KeyEvent::Alt(c) => write!(f, "Alt-{}", c),
                KeyEvent::Control(c) => write!(f, "Ctrl-{}", c),
                KeyEvent::Char(c) => write!(f, "{}", c),
                KeyEvent::F(c) => write!(f, "F{}", c),
                KeyEvent::Enter => write!(f, "Enter"),
                KeyEvent::Backspace => write!(f, "Backspace"),
                KeyEvent::Tab => write!(f, "Tab"),
                KeyEvent::Escape => write!(f, "Escape"),
                KeyEvent::Up => write!(f, "Up Arrow"),
                KeyEvent::Down => write!(f, "Down Arrow"),
                KeyEvent::Right => write!(f, "Right Arrow"),
                KeyEvent::Left => write!(f, "Left Arrow"),
            },
            Event::Mouse(mouse_event) => {
                let btn = |data: &MouseData| -> &str {
                    match data.button {
                        MouseButton::Left => "Left",
                        MouseButton::Right => "Right",
                        MouseButton::Middle => "Middle",
                        MouseButton::Fourth => "Fourth",
                        MouseButton::Fith => "Fith",
                        MouseButton::None => "Unknown",
                    }
                };
                match mouse_event {
                    MouseEvent::Move(data) => {
                        if data.button == MouseButton::None {
                            write!(f, "Moved({},{})", data.pos.x, data.pos.y)
                        } else {
                            write!(f, "Dragged({},{},{})", btn(data), data.pos.x, data.pos.y)
                        }
                    }
                    MouseEvent::Pressed(data) => {
                        write!(f, "{} Button Pressed", btn(data))
                    }
                    MouseEvent::Release(data) => {
                        write!(f, "{} Button Released", btn(data))
                    }
                }
            }
            Event::Other(other_event) => match other_event {
                OtherEvent::EnterFocus => write!(f, "Enter Focus Event"),
                OtherEvent::LeaveFocus => write!(f, "Leave Focus Event"),
                OtherEvent::ScreenSizeChange { width, height } => {
                    write!(f, "Screen Size changed to ({},{})", width, height)
                }
                OtherEvent::Unknown(s) => write!(f, "Unknown ({})", s),
            },
            _ => write!(f, "Unhandled Input Event"),
        }
    }
}
#[cfg(windows)]
pub mod windows {}
