mod state;
mod line;

extern crate termion;

use state::*;
use line::*;

use termion::{
    color::{
        Fg,
        Red,
        Reset
    }
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
    }
};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let l = Box::new(Line::new("foo".to_string()));
    let m = Box::new(Line::new("bang arrr".to_string()));
    let mut state = State::new();
    let id1 = state.push(l);
    let id2 = state.push(m);

    state.before(id1, Box::new(|writer: &mut Write| {
        write!(writer, "{}", Fg(Red))
    }));

    state.after(id1, Box::new(|writer: &mut Write| {
        write!(writer, "{}", Fg(Reset))
    }));

    state.render(&mut writer).unwrap();
    writer.flush().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
}
