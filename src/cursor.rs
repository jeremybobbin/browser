use crate::state;
use crate::line;

extern crate termion;

use state::*;
use line::*;

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
    }
};
// ID refers to ID of renderable 'selection'
pub struct Cursor{
    id: ID
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            id: 1
        }
    }
    pub fn pre(&self, state: &mut State) {
        state.before(self.id, Box::new(|writer: &mut Write| {
            write!(writer, "{}", Fg(Red))
        }));

        state.after(self.id, Box::new(|writer: &mut Write| {
            write!(writer, "{}", Fg(Reset))
        }));
    }

    pub fn post(&self, state: &mut State) {
        state.clear(self.id);
    }

    pub fn handle(&mut self, key: &Key) {
        match key {
            Key::Char('j') => self.id += 1,
            Key::Char('k') => self.id -= 1,
            _ => {}
        }
    }
}
