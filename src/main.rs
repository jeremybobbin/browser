mod state;
mod renderer;
mod cursor;
mod entry;
mod manager;
mod types;

extern crate termion;

use manager::*;

use termion::{
    input::TermRead,
    raw::IntoRawMode,
};


use std::{
    io::{
        self,
        BufWriter
    },
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
