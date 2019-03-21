extern crate termion;

use crate::{
    state::*,
    renderer::*,    
    cursor::*,
    types::*,
};


use termion::{
    event::Key,
    cursor::{
        Goto
    },
    clear
    
};

use std::{
    io::{
        self,
        Write
    },
    rc::Rc,
    env,
    fs
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
            alive: true,
        })
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn handle_key(&mut self, key: &Key) {
        if let termion::event::Key::Char('q') = key {
            self.alive = false;
        }
        match key {
            Key::Char('q') => self.alive = false,
            Key::Char('h') => {
                env::set_current_dir("..");
                self.swap(Rc::new(State::new().unwrap()));
            },
            Key::Char('l') => {
                let id = self.cursor.get();
                let dir = self.state.get(id).unwrap().select();
                self.swap(Rc::new(State::new().unwrap()));
            },
            _ => {},

        }
        self.cursor.handle(key)
    }

    pub fn render(&mut self, writer: &mut Screen) -> io::Result<()> {
        let Manager{ ref cursor, ref mut renderer, ref state, .. } = self;
        write!(writer, "{}{}", clear::All, Goto(1, 1))?;
        cursor.before(renderer);
        renderer.render(writer)?;
        cursor.after(renderer);
        writer.flush()
    }
}

impl Viewer for Manager {
    fn swap(&mut self, state: Rc<State>) {
        self.  cursor.swap(Rc::clone(&state));
        self.renderer.swap(Rc::clone(&state));
        self.state = state
    }
}
