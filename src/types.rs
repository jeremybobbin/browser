extern crate termion;

use std::{
    io::{
        Result,
        BufWriter,
        Stdout,
        Write
    }
};

use termion::raw::RawTerminal;

pub trait Renderable {
    fn render(&self, writer: &mut Write) -> Result<()>;
}

pub type Screen = BufWriter<RawTerminal<Stdout>>;
pub type Render = Box<Fn(&mut Write) -> Result<()>>;
pub type ID = usize;

