extern crate termion;

use std::{
    io::{
        Result,
        BufWriter,
        Stdout,
        Write
    },
    rc::Rc
};

use crate::{
    state::*,
    renderer::*,
};

use termion::raw::RawTerminal;


pub trait Renderable {
    fn render(&self, writer: &mut Write) -> Result<()>;
    fn select(&self);
}

// Things that modify themselves based on state
//   - State should really just be called 'Components'
//  
//  When state ref changes, viewers need to swap their state.
pub trait Viewer {
    fn swap(&mut self, state: Rc<State>);
}

// Things that affect rendering but *not* state are 'effective' like 'special effects' 
// They do a thing before a render, and usually take away the thing after render.
pub trait Affector {
    fn before(&self, renderer: &mut Renderer);
    fn  after(&self, renderer: &mut Renderer);
}

pub type Screen = BufWriter<RawTerminal<Stdout>>;
pub type Render = Box<Fn(&mut Write) -> Result<()>>;
pub type ID = usize;

