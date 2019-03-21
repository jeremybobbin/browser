extern crate termion;

use crate::state;

use state::*;

use termion::{
    color::{
        Blue,
        Red,
        Magenta,
        Reset,
        Fg
    },
    cursor::{
        Goto,
        Left,
        Down,
    },
    style::{
        self,
        Bold,
        NoBold
    }
};

use std::{
    io::{
        self,
        Write
    },
    fs::{
        self,
        DirEntry
    },
    ffi::OsString
};

impl Renderable for DirEntry {
    fn render(&self, writer: &mut Write) -> io::Result<()> {
        let name = self.file_name()
            .into_string()
            .unwrap_or("NULL".to_string());

        let file_type = self.file_type()?;
        if file_type.is_file() {
            write!(writer, "{}{}{}", name, Down(1), Left(name.len() as u16))
        } else if file_type.is_dir() {
            write!(writer, "{}{}{}{}{}", Fg(Blue), name, Fg(Reset), Down(1), Left(name.len() as u16))
        } else {
            write!(writer, "{}{}{}{}{}", Fg(Magenta), name, Fg(Reset), Down(1), Left(name.len() as u16))
        }
    }
}
