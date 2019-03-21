use crate::line;
use crate::renderer;
use crate::state;

extern crate termion;

use line::*;
use state::*;
use renderer::*;

use termion::{
    color::{
        Reset,
        Fg,
        Red
    },
    event::{
        Event,
        Key
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
    id: ID,
    state: Rc<State>
}

impl Cursor {
    pub fn new(state: Rc<State>) -> Cursor {
        Cursor {
            id: 1,
            state
        }
    }

    pub fn pre(&self, renderer: &mut Renderer) {
        renderer.before(self.id, Box::new(|writer: &mut Write| {
            write!(writer, "{}", Fg(Red))
        }));

        renderer.after(self.id, Box::new(|writer: &mut Write| {
            write!(writer, "{}", Fg(Reset))
        }));
    }

    pub fn post(&self, renderer: &mut Renderer) {
        renderer.clear(self.id);
    }

    pub fn incr(&mut self, delta: i32) {
        let max = self.state.max() as i32;
        let mut new_id = self.id as i32 + delta;
        if new_id > max {
            new_id = max;
        }
        if new_id < 0 {
            new_id = 0;
        }
        self.id = new_id as ID;
    }

    pub fn swap(&mut self, state: Rc<State>) {
        self.state = state;
    }

    pub fn handle(&mut self, key: &Key) {
        match key {
            Key::Char('j') => self.incr( 1),
            Key::Char('k') => self.incr(-1),
            _ => {}
        }
    }
}
