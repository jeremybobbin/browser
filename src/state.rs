use crate::{
    types::*,
};

use std::{
    io,
    fs,
    ops::{
        Deref,
        DerefMut
    },
};

pub struct State(Vec<Box<dyn Renderable>>);

impl Deref for State {
	type Target = Vec<Box<dyn Renderable>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for State {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl State {
    pub fn new() -> io::Result<State> {
        let entries: Vec<Box<dyn Renderable>> = fs::read_dir(".")?
            .filter_map(|e| e.ok())
            .map(|e| Box::new(e) as Box<dyn Renderable>)
            .collect();

        Ok(State(entries))
    }

    pub fn push(&mut self, item: Box<dyn Renderable>) -> ID {
        let index = self.0.len();
        self.0.push(item);
        index
    }

    pub fn has(&self, id: ID) -> bool {
        self.len() > id && id >= 0
    }
}
