mod state;
mod renderer;
mod line;
mod cursor;
mod entry;
mod manager;

extern crate termion;

use state::*;
use line::*;
use cursor::*;
use renderer::*;
use entry::*;
use manager::*;


use termion::{
    color::{
        Fg,
        Red,
        Reset
    },
    clear,
    cursor::{
        Goto
    },
    input::TermRead,
    raw::IntoRawMode,
};

use std::{
    ops::{
        Deref,
        DerefMut
    },
    io::{
        self,
        Write,
        BufWriter
    },
    rc::Rc,
};

fn render() -> io::Result<()> {
    let mut stdout = io::stdout().into_raw_mode()?;
    let mut stdin  = io::stdin();

    let mut manager  = Manager::new()?;
    let mut writer = BufWriter::new(stdout);
    let mut keys   = stdin.keys()
        .filter_map(Result::ok);


    manager.render(&mut writer);
    for key in keys {
        manager.handle_key(&key);
        if !manager.is_alive() {
            break;
        }
        manager.render(&mut writer);
    }

    Ok(())
}

fn main() {
    render().unwrap();
}
