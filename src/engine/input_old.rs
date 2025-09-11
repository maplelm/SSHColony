pub const MAX_INPUT_LEN: usize = 1024;

#[cfg(unix)]
pub mod unix {
    pub const UP_ARROW   : &str = "\x1b[A";
    pub const DOWN_ARROW : &str = "\x1b[B";
    pub const RIGHT_ARROW: &str = "\x1b[C";
    pub const LEFT_ARROW : &str = "\x1b[D";
    pub const Q: char = 'q';

    pub const SHIFT_Q: char ='Q';
}

#[cfg(windows)]
pub mod windows {

}