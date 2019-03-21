mod state;
mod renderer;
mod line;
mod cursor;
mod entry;

extern crate termion;

use state::*;
use line::*;
use cursor::*;
use renderer::*;
use entry::*;


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
    let mut stdin    = io::stdin();
    let mut stdout   = io::stdout().into_raw_mode()?;

    let mut writer   = BufWriter::new(stdout);
    let mut state    =     State::new().unwrap();

    let l = Box::new(Line::new("foo".to_string()));
    let m = Box::new(Line::new("bang arrr".to_string()));

    let id1 = state.push(l);
    let id2 = state.push(m);

    let state = Rc::new(state);

    let mut cursor   =    Cursor::new(Rc::clone(&state));
    let mut renderer =  Renderer::new(Rc::clone(&state));

    renderer.before(id1, Box::new(|writer: &mut Write| {
        write!(writer, "{}", Fg(Red))
    }));

    renderer.after(id1, Box::new(|writer: &mut Write| {
        write!(writer, "{}", Fg(Reset))
    }));

    for c in stdin.keys() {
        write!(writer, "{}{}", clear::All, Goto(1, 1))?;
        println!("{:?}", c);
        let c = c.unwrap();
        if let termion::event::Key::Char('q') = c {
            break;
        }
        cursor.handle(&c);
        cursor.pre(&mut renderer);
        renderer.render(&mut writer)?;
        cursor.post(&mut renderer);
        writer.flush()?;
    }
    Ok(())
}

fn main() {
    render().unwrap();
}
