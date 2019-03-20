extern crate termion;
use termion::{
    cursor::{
        Goto
    },
    clear,
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
    pub out: BufWriter<Stdout>,
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
    ($name:ident, $type:expr) => {
        pub fn $name(&mut self) -> io::Result<()> {
            write!(self, "{}", Fg($type))
        }
    }
}

impl Printer {
    pub fn new(stdout: Stdout) -> Self {
        Printer {
            out: BufWriter::new(stdout),
            x: 1,
            y: 1
        }
    }
    pub fn endln(&mut self) -> io::Result<()> {
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

    pub fn reset(&mut self) -> io::Result<()> {
        write!(self, "{}{}", clear::All, Goto(1, 1))
    }
    impl_color!(cyan, Cyan);
    impl_color!(yellow, Yellow);
    impl_color!(red, Red);
    impl_color!(green, Green);
    impl_color!(blue, Blue);
    impl_color!(regular, Reset);
}
