extern crate termion;

use crate::state;

use state::*;

use termion::{
    color::{
        Red,
        Reset,
        Fg
    },
    cursor::{
        Goto,
        Left,
        Down,
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
        write!(writer, "{}{}{}", name, Down(1), Left(name.len() as u16))
    }
}
