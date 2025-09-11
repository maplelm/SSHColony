#![allow(dead_code)]
#![allow(invalid_value)]

#[cfg(unix)]
pub mod unix {
    use std::{mem, os::fd::AsRawFd};
    use std::io::stdin;
    use libc:: { c_uchar, c_uint, tcflag_t, tcgetattr, tcsetattr, termios, NCCS };

    pub const STDOUT_BUFFER_SIZE: usize = 2048; // bytes

    pub type Lflag = tcflag_t;
    pub type Iflag = tcflag_t;
    pub type Oflag = tcflag_t;
    pub type Cflag = tcflag_t;
    pub type SCharMap = [u8; NCCS];
    pub type SChar = u8;
    pub type TcFlow = i32;
    pub type TcFlush = i32;
    pub type TcSet = i32;


    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(C)]
    pub struct Terminal {
        pub c_iflags: Iflag, // input flags
        pub c_oflags: Oflag, // output flags
        pub c_cflags: Cflag, // control falgs
        pub c_lflags: Lflag, // local flags
        pub c_cc: SCharMap,
        c_line: c_uchar,
        c_ispeed: c_uint,
        c_ospeed: c_uint,
        alt_buffer: bool,
        cursor_visable: bool
    }

    impl Default for Terminal {
        fn default() -> Self {
            unsafe {
                let mut t : termios = mem::MaybeUninit::uninit().assume_init();
                let res = tcgetattr(stdin().as_raw_fd(), &mut t);
                if res == 0 {
                    Self {
                        c_iflags: t.c_iflag,
                        c_oflags: t.c_oflag,
                        c_lflags: t.c_lflag,
                        c_cflags: t.c_cflag,
                        c_line: t.c_line,
                        c_cc: t.c_cc,
                        c_ispeed: t.c_ispeed,
                        c_ospeed: t.c_ospeed,
                        alt_buffer: false,
                        cursor_visable: true,
                    }
                }
                else {
                    Self {
                        c_iflags: 0,
                        c_oflags: 0,
                        c_lflags: 0,
                        c_cflags: 0,
                        c_line: 0,
                        c_cc: [0; NCCS],
                        c_ispeed: 0,
                        c_ospeed: 0,
                        alt_buffer: false,
                        cursor_visable: true,
                    }
                }
            }
        }
    }

    impl Terminal {
        pub fn cast_to_termios(&self) -> termios {
            termios {
                c_cc: self.c_cc,
                c_cflag: self.c_cflags,
                c_lflag: self.c_lflags,
                c_iflag: self.c_iflags,
                c_oflag: self.c_oflags,
                c_line: self.c_line,
                c_ispeed: self.c_ispeed,
                c_ospeed: self.c_ospeed,
            }
        }

        pub fn cursor_visable(&self) -> bool {
            return self.cursor_visable;
        }

        pub fn alt_buffer(&self) -> bool {
            return self.alt_buffer;
        }

        pub fn toggle_cursor_visable(&mut self) -> bool {
            if self.cursor_visable {
                print!("\x1b[?25l");
            } else {
                print!("\x1b[?25h");
            }
            self.cursor_visable = !self.cursor_visable;
            return self.cursor_visable;
        }

        pub fn toggle_alt_buffer(&mut self) -> bool {
            if self.alt_buffer {
                //ESC[?1049l
            } else {
                print!("\x1b[?1049h\x1b[2J");
                //ESC[?1049h
            }
            self.alt_buffer = !self.alt_buffer;
            return self.alt_buffer;
        }
    }

    // c_cc characters
    pub const VINTR:    SChar = 0;
    pub const VQUIT:    SChar = 1;
    pub const VERASE:   SChar = 2;
    pub const VKILL:    SChar = 3;
    pub const VEOF:     SChar = 4;
    pub const VTIME:    SChar = 5;
    pub const VMIN:     SChar = 6;
    pub const VSWTC:    SChar = 7;
    pub const VSTART:   SChar = 8;
    pub const VSTOP:    SChar = 9;
    pub const VSUSP:    SChar = 10;
    pub const VEOL:     SChar = 11;
    pub const VREPRINT: SChar = 12;
    pub const VDISCARD: SChar = 13;
    pub const VWERASE:  SChar = 14;
    pub const VLNEXT:   SChar = 15;
    pub const VEOL2:    SChar = 16;

    // c_lflag bits
    pub const ISIG:    Lflag  = 0o000001;
    pub const ICANON:  Lflag  = 0o000002;
    pub const XCASE:   Lflag  = 0o000004;
    pub const ECHO:    Lflag  = 0o000010;
    pub const ECHOE:   Lflag  = 0o000020;
    pub const ECHOK:   Lflag  = 0o000040;
    pub const ECHONL:  Lflag  = 0o000100;
    pub const NOFLSH:  Lflag  = 0o000200;
    pub const TOSTOP:  Lflag  = 0o000400;
    pub const ECHOCTL: Lflag  = 0o001000;
    pub const ECHOPRT: Lflag  = 0o002000;
    pub const ECHOKE:  Lflag  = 0o004000;
    pub const FLUSHO:  Lflag  = 0o010000;
    pub const PENDIN:  Lflag  = 0o040000;
    pub const IEXTEN:  Lflag  = 0o100000;
    pub const EXTPROC: Lflag = 0o200000;

    // c_oflag bits
    pub const OPOST: Oflag = 0x00000001;

    // c_iflag bits
    pub const IXON: Iflag = libc::IXON;

    // TCFLOW
    pub const TCOOFF: TcFlow = 0;
    pub const TCOON:  TcFlow = 1;
    pub const TCIOFF: TcFlow = 2;
    pub const TCION:  TcFlow = 3;

    // TCFLUSH
    pub const TCIFLUSH:  TcFlush  = 0;
    pub const TCOFLUSH:  TcFlush  = 1;
    pub const TCIOFLUSH: TcFlush = 2;

    // TCSETATTR
    pub const TCSANOW:   TcSet = 0;
    pub const TCSADRAIN: TcSet = 1;
    pub const TCSAFLUSH: TcSet = 2;


    pub fn set_term(t: Terminal) -> Terminal {
        let original= Terminal::default();
        unsafe {tcsetattr(stdin().as_raw_fd(), TCSANOW, &t.cast_to_termios());}
        return original;
    }

    // If nothing is set the function will set the flag to what it would be in raw mode.
    // Returns the terminal settings before raw mode was enabled
    pub fn set_raw() -> Terminal {
        let t: Terminal = Terminal::default();
        unsafe {
            let mut termios: libc::termios = mem::MaybeUninit::uninit().assume_init();
            tcgetattr(stdin().as_raw_fd(), &mut termios);
            termios.c_lflag &= !(ICANON | ECHO | ISIG);
            termios.c_iflag &= !(IXON);
            termios.c_oflag &= !(OPOST);
            tcsetattr(stdin().as_raw_fd(), TCSANOW, &termios);
        }
        return t;
    }
    pub fn set_flags(lflags: Option<Lflag>, iflags: Option<Iflag>, oflags: Option<Oflag>, cflags: Option<Cflag>) {
        unsafe {
            let mut t: libc::termios = mem::MaybeUninit::uninit().assume_init();
            tcgetattr(stdin().as_raw_fd(), &mut t);
            t.c_iflag = if let Some(flag) = iflags{flag} else{t.c_iflag};
            t.c_lflag = if let Some(flag) = lflags{flag} else {t.c_lflag};
            t.c_oflag = if let Some(flag) = oflags{flag} else {t.c_oflag};
            t.c_cflag = if let Some(flag) = cflags{flag} else {t.c_cflag};
            tcsetattr(stdin().as_raw_fd(),TCSANOW, &mut t);
        }
    }
}

#[cfg(windows)]
pub mod windows {

}