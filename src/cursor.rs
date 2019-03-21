use crate::{
    types::*,
    renderer::*,
    state::*,
};

extern crate termion;

use termion::{
    color::{
        Reset,
        White,
        Fg,
        Bg,
        Red
    },
    event::{
        Event,
        Key
    },
    style::{
        Bold,
    }
};

use std::{
    io::{
        Write
    },
    rc::Rc
};
// ID refers to ID of renderable 'selection'
pub struct Cursor{
    id:    ID,
    state: Rc<State>
}

impl Viewer for Cursor {
    fn swap(&mut self, state: Rc<State>) {
        self.state = state;
    }
}

impl Cursor {
    pub fn new(state: Rc<State>) -> Cursor {
        Cursor {
            id: 1,
            state
        }
    }

    pub fn incr(&mut self, delta: i32) {
        let max = self.state.len() as i32 - 1;
        let mut new = self.id as i32 + delta;
        if new > max {
            self.id = max as ID;
        } else if new < 0 {
            self.id = 0;
        } else {
            self.id = new as ID;
        }
    }

    pub fn handle(&mut self, key: &Key) {
        match key {
            Key::Char('j') => self.incr( 1),
            Key::Char('k') => self.incr(-1),
            _ => {}
        }
    }

    pub fn get(&self) -> ID {
        self.id
    }
}

impl Affector for Cursor {
    fn before(&self, renderer: &mut Renderer) {
        renderer.before(self.id, Box::new(|writer: &mut Write| {
            write!(writer, "{}", Bg(White))
        }));

        renderer.after(self.id, Box::new(|writer: &mut Write| {
            write!(writer, "{}", Bg(Reset))
        }));
    }

    fn after(&self, renderer: &mut Renderer) {
        renderer.clear(self.id);
    }
}
