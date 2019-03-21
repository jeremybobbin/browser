use crate::{
    entry::*,
    types::*,
};

use std::{
    collections::HashMap,
    io::{
        self,
        Stdout,
        Write
    },
    ops::{
        Deref,
        DerefMut
    },
    fs,
    ffi::OsString
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
            .map(|e| {
                let e: Box<dyn Renderable> = Box::new(e);
                e
            })
            .collect();

        Ok(State(entries))
    }
    
    pub fn push(&mut self, item: Box<dyn Renderable>) -> ID {
        let index = self.0.len();
        self.0.push(item);
        index
    }

    pub fn max(&self) -> ID {
        self.len() - 1
    }
}
