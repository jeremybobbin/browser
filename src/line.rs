extern crate termion;
use termion::{
    cursor::{
        Goto,
        Down,
        Left
    }
};

use crate::state;
use state::*;

use std::{
    ops::{
        Deref,
        DerefMut
    },
    io::{
        self,
        Write
    }
};

pub struct Line(String);

impl Line {
    pub fn new(s: String) -> Line {
        Line(s)
    }
}

impl Deref for Line {
	type Target = String;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Line {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Renderable for Line {
    fn render(&self, writer: &mut Write) -> io::Result<()> {
        write!(writer, "{}{}{}", self.0, Down(1), Left(self.len() as u16))
    }
}
