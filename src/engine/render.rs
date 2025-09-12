use std::{
    cell::{Ref, RefCell},
    mem::swap,
    rc::{Rc, Weak},
    sync::mpsc,
    time::{Duration, Instant},
};

//////////////////////
///  TYPE ALIASES  ///
//////////////////////

pub type SpritePrefix = Option<String>;
pub type SpriteSufix = Option<String>;
pub type Sprite = String;

///////////////
///  ENUMS  ///
///////////////

#[derive(Clone)]
pub enum Object {
    Static(StaticObject),
    Dynamic(DynamicObject),
}

pub enum RenderMsg {
    Insert(ObjectPos, Object),
    InsertRange(ObjectPos, ObjectPos, Object),
    InsertText(ObjectPos, Sprite, SpritePrefix, SpriteSufix),
    Remove(ObjectPos),
    RemoveRange(ObjectPos, ObjectPos),
    Swap(ObjectPos, ObjectPos),
    Batch(Vec<RenderMsg>),
    Clear,
}

/////////////////
///  OBJECTS  ///
/////////////////

pub struct Canvas {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone)]
pub struct StaticObject {
    sprite: Sprite,
}

#[derive(Clone)]
pub struct DynamicObject {
    pub sprite: Vec<Sprite>,
    pub cursor: usize,
    pub tick: Duration,

    last_tick: Instant,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ObjectPos {
    pub x: u32,
    pub y: u32,
}

pub struct ObjectMove {
    pub old: ObjectPos,
    pub new: ObjectPos,
}

/////////////////////////
///  IMPLEMENTATIONS  ///
/////////////////////////

impl Object {
    pub fn as_dynamic(&mut self) -> Option<&mut DynamicObject> {
        if let Object::Dynamic(v) = self {
            return Some(v);
        }
        return None;
    }
    pub fn as_static(&mut self) -> Option<&mut StaticObject> {
        if let Object::Static(v) = self {
            return Some(v);
        }
        return None;
    }

    pub fn sprite(&self) -> &str {
        match self {
            Object::Dynamic(d) => &(d.sprite[d.cursor]),
            Object::Static(s) => &s.sprite,
        }
    }
}

impl StaticObject {
    pub fn new(s: &str) -> Option<Self> {
        print!("\x1b[50;35flen: {} \n\r", s.len());
        if s.len() >= 1 {
            return Some(Self {
                sprite: String::from(s),
            });
        }
        return None;
    }
    pub fn sprite(&self) -> &String {
        return &self.sprite;
    }
}

impl DynamicObject {
    pub fn new(s: Vec<String>, tick_rate: Duration) -> Option<Self> {
        if s.len() == 0 {
            return None;
        }
        for each in s.iter() {
            if each.len() < 1 {
                return None;
            }
        }
        Some(Self {
            sprite: s,
            cursor: 0,
            tick: tick_rate,
            last_tick: Instant::now(),
        })
    }
    pub fn sprite(&self) -> &String {
        return &self.sprite[self.cursor];
    }

    // Returns true of update changes the sprite
    pub fn update(&mut self) -> bool {
        if Instant::now().duration_since(self.last_tick) >= self.tick {
            if self.cursor == self.sprite.len() - 1 {
                self.cursor = 0;
            } else {
                self.cursor += 1;
            }
            self.last_tick = Instant::now();
            return true;
        }
        return false;
    }
}

//////////////////
/// FUNCTIONS  ///
//////////////////

pub fn render_msg_disbatch(
    msg: RenderMsg,
    canvas: &Canvas,
    obj_buff: &mut Vec<Option<Rc<RefCell<Object>>>>,
    dyn_buff: &mut Vec<Weak<RefCell<Object>>>,
) {
    match msg {
        RenderMsg::Batch(b) => {
            for each in b {
                render_msg_disbatch(each, &canvas, obj_buff, dyn_buff);
            }
        }
        RenderMsg::Insert(pos, obj) => match obj {
            Object::Static(_) => {
                obj_buff[pos.y as usize * canvas.width + pos.x as usize] =
                    Some(Rc::new(RefCell::new(obj)));
            }
            Object::Dynamic(_) => {
                let val = Rc::new(RefCell::new(obj));
                dyn_buff.push(Rc::downgrade(&val));
                obj_buff[pos.y as usize * canvas.width + pos.x as usize] = Some(val);
            }
        },
        RenderMsg::InsertRange(start, end, object) => {
            let start_pos = start.y as usize * canvas.width + start.x as usize;
            match object {
                Object::Static(_) => {
                    for y in 0..(start.y as i32 - end.y as i32).abs() {
                        for x in 0..(start.x as i32 - end.x as i32).abs() {
                            obj_buff[start_pos + (y as usize * canvas.width) + x as usize] =
                                Some(Rc::new(RefCell::new(object.clone())));
                        }
                    }
                }
                Object::Dynamic(_) => {
                    for y in 0..(start.y as i32 - end.y as i32).abs() {
                        for x in 0..(start.x as i32 - end.x as i32).abs() {
                            obj_buff[start_pos + (y as usize * canvas.width) + x as usize] =
                                Some(Rc::new(RefCell::new(object.clone())));
                            dyn_buff.push(Rc::downgrade(
                                obj_buff[start_pos + (y as usize * canvas.width) + x as usize]
                                    .as_ref()
                                    .unwrap(),
                            ));
                        }
                    }
                }
            }
        }
        RenderMsg::InsertText(pos, s, prefix, sufix) => {
            let mut y: usize = pos.y as usize;
            let mut x: usize = pos.x as usize;
            for each in s.chars() {
                if each == ' ' {
                    obj_buff[y * canvas.width + x] = None;
                } else {
                    let mut sprite = String::from(each);
                    if let Some(prefix) = &prefix {
                        sprite.insert_str(0, prefix);
                    }
                    if let Some(sufix) = &sufix {
                        sprite.push_str(sufix);
                    }
                    obj_buff[y * canvas.width + x] =
                        Some(Rc::new(RefCell::new(Object::Static(StaticObject {
                            sprite: sprite,
                        }))));
                }
                if each == '\n' {
                    y += 1;
                    x = pos.x as usize;
                } else {
                    x += 1;
                }
            }
        }
        RenderMsg::Remove(pos) => {
            obj_buff[pos.y as usize * canvas.width + pos.x as usize] = None;
        }
        RenderMsg::RemoveRange(start, end) => {
            let cursor: usize = start.y as usize * canvas.width + start.x as usize;
            for y in 0..(start.y as i32 - end.y as i32).abs() {
                for x in 0..(start.x as i32 - end.x as i32).abs() {
                    obj_buff[cursor + (y as usize * canvas.width) + (x as usize)] = None;
                }
            }
        }
        RenderMsg::Swap(a, b) => {
            obj_buff.swap(
                a.y as usize * canvas.width + a.x as usize,
                b.y as usize * canvas.height + b.x as usize,
            );
        }
        RenderMsg::Clear => {
            obj_buff.resize(canvas.height * canvas.width, None);
            dyn_buff.clear();
        }
        _ => {
            todo!()
        }
    }
}

pub fn insert_plain_text(
    x: u32,
    y: u32,
    s: String,
    ch: &mpsc::Sender<RenderMsg>,
) -> Result<(), mpsc::SendError<RenderMsg>> {
    ch.send(RenderMsg::InsertText(
        ObjectPos { x: x, y: y },
        s,
        None,
        None,
    ))
}

pub fn insert_object(
    x: u32,
    y: u32,
    obj: Object,
    ch: &mpsc::Sender<RenderMsg>,
) -> Result<(), mpsc::SendError<RenderMsg>> {
    ch.send(RenderMsg::Insert(ObjectPos { x: x, y: y }, obj))
}

pub mod sprites {
    use super::colors::*;
    use std::time::{Duration, Instant};

    use super::{DynamicObject, Object, StaticObject};

    pub fn dwarf_object() -> Object {
        Object::Static(StaticObject {
            sprite: format!("{}D", ISO_WHITE_FOREGROUND),
        })
    }

    pub fn hurt_dwarf_object(millis: u64) -> Object {
        Object::Dynamic(DynamicObject {
            sprite: vec![
                format!("{}D", ISO_RED_FOREGROUND),
                format!("{}D", ISO_WHITE_FOREGROUND),
            ],
            tick: Duration::from_millis(millis),
            cursor: 0,
            last_tick: Instant::now(),
        })
    }
}

pub mod colors {
    use std::fmt::Display;

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct Mode(u8);
    impl Mode {
        pub fn new(s: u8) -> Option<Self> {
            if (s > 9 && s < 22) || s == 6 || s == 26 || s > 29 {
                None
            } else {
                Some(Self(s))
            }
        }
    }
    impl Display for Mode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "\x1b[{}m", self.0)
        }
    }

    pub const BOLD_MODE_ON: Mode = Mode(1);
    pub const DIM_MODE_ON: Mode = Mode(2);
    pub const ITALICS_MODE_ON: Mode = Mode(3);
    pub const UNDERLINE_MODE_ON: Mode = Mode(4);
    pub const INVERSE_MODE_ON: Mode = Mode(7);
    pub const STRIKETHROUGH_MODE_ON: Mode = Mode(9);

    pub const BOLD_MODE_OFF: Mode = Mode(22);
    pub const DIM_MODE_OFF: Mode = Mode(22);
    pub const ITALICS_MODE_OFF: Mode = Mode(23);
    pub const UNDERLINE_MODE_OFF: Mode = Mode(24);
    pub const INVERSE_MODE_OFF: Mode = Mode(27);
    pub const STRIKETHROUGH_MODE_OFF: Mode = Mode(29);

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum Color {
        IsoForeground(u8),
        IsoBackground(u8),
        ExtendedForeground(u8),
        ExtendedBackground(u8),
        RgbForeground(u8, u8, u8),
        RgbBackground(u8, u8, u8),
    }

    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Color::IsoForeground(c) => write!(f, "\x1b[3{}m", c),
                Color::IsoBackground(c) => write!(f, "\x1b[4{}m", c),
                Color::ExtendedForeground(c) => write!(f, "\x1b[38;5;{}m", c),
                Color::ExtendedBackground(c) => write!(f, "\x1b[48;5;{}m", c),
                Color::RgbForeground(r, g, b) => write!(f, "\x1b[38;2;{};{};{}m", r, g, b),
                Color::RgbBackground(r, g, b) => write!(f, "\x1b[48;2;{};{};{}m", r, g, b),
            }
        }
    }

    pub const ISO_BLACK_FOREGROUND: Color = Color::IsoForeground(0);
    pub const ISO_BLACK_BACKGROUND: Color = Color::IsoBackground(0);
    pub const ISO_RED_FOREGROUND: Color = Color::IsoForeground(1);
    pub const ISO_RED_BACKGROUND: Color = Color::IsoBackground(1);
    pub const ISO_GREEN_FOREGROUND: Color = Color::IsoForeground(2);
    pub const ISO_GREEN_BACKGROUND: Color = Color::IsoBackground(2);
    pub const ISO_YELLOW_FOREGROUND: Color = Color::IsoForeground(3);
    pub const ISO_YELLOW_BACKGROUND: Color = Color::IsoBackground(3);
    pub const ISO_BLUE_FOREGROUND: Color = Color::IsoForeground(4);
    pub const ISO_BLUE_BACKGROUND: Color = Color::IsoBackground(4);
    pub const ISO_MAGENTA_FOREGROUND: Color = Color::IsoForeground(5);
    pub const ISO_MAGENTA_BACKGROUND: Color = Color::IsoBackground(5);
    pub const ISO_CYAN_FOREGROUND: Color = Color::IsoForeground(6);
    pub const ISO_CYAN_BACKGROUND: Color = Color::IsoBackground(6);
    pub const ISO_WHITE_FOREGROUND: Color = Color::IsoForeground(7);
    pub const ISO_WHITE_BACKGROUND: Color = Color::IsoBackground(7);

    pub const COLOR_RESET: &'static str = "\x1b[0m";
}
