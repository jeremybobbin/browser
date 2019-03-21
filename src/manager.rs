extern crate termion;

use crate::{
    state::*,
    renderer::*,    
    cursor::*,
    types::*,
};


use termion::{
    input::TermRead,
    raw::{
        RawTerminal,
        IntoRawMode,
    },
    event::Key,
    cursor::{
        Goto
    },
    clear
    
};

use std::{
    ops::{
        Deref,
        DerefMut
    },
    io::{
        self,
        Stdin,
        Stdout,
        Write,
        BufWriter,
    },
    rc::Rc,
};


pub struct Manager {
    cursor  : Cursor,
    renderer: Renderer,
    state   : Rc<State>,
    alive   : bool
}

impl Manager {
    pub fn new() -> io::Result<Manager> {
        let mut state    = Rc::new(State::new()?);
        let mut cursor   = Cursor::new(Rc::clone(&state));
        let mut renderer = Renderer::new(Rc::clone(&state));

        Ok(Manager {
            cursor,
            renderer,
            state,
            alive: true
        })
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn handle_key(&mut self, key: &Key) {
        if let termion::event::Key::Char('q') = key {
            self.alive = false;
        }
        self.cursor.handle(key)
    }

    pub fn render(&mut self, writer: &mut Screen) -> io::Result<()> {
        let Manager{ ref cursor, ref mut renderer, ref state, .. } = self;
        write!(writer, "{}{}", clear::All, Goto(1, 1))?;
        cursor.pre(renderer);
        renderer.render(writer)?;
        cursor.post(renderer);
        writer.flush()
    }
}
