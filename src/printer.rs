extern crate termion;
use termion::{
    cursor::{
        Goto
    },
    color::{
        Fg,
        Bg,
        Red,
        Yellow,
        Green,
        Blue,
        Cyan,
        Reset,
    }

};
use std::{
    io::{
        self,
        Stdout,
        Result,
        BufWriter,
        prelude::*
    },
    ops::{
        Deref,
        DerefMut
    }
};

pub struct Printer{
    out: BufWriter<Stdout>,
    x: u16,
    y: u16
}

impl DerefMut for Printer {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.out
	}
}


impl Deref for Printer {
	type Target = BufWriter<Stdout>;
	fn deref(&self) -> &Self::Target {
		&self.out
	}
}

macro_rules! impl_color {
    ($e:ident) => {
        pub fn $e(&mut self, contents: &str) -> io::Result<()> {
            write!(self, "{}{}{}", Fg($e), contents, Fg(Reset))
        }
    }
}

impl Printer {
    pub fn new() -> Self {
        Printer {
            out: BufWriter::new(io::stdout()),
            x: 0,
            y: 0
        }
    }
    fn endln(&mut self) -> io::Result<()> {
        let new_y = self.y + 1;
        let res = write!(self, "{}", Goto(1, new_y));
        if res.is_ok() {
            self.y = new_y
        };
        res
    }
    pub fn put(&mut self, contents: &str) -> io::Result<()> {
        write!(self, "{}", contents)
    }
    impl_color!(Cyan);
    impl_color!(Yellow);
    impl_color!(Red);
    impl_color!(Green);
    impl_color!(Blue);
}
