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

pub type ID = usize;
pub type Render = Box<Fn(&mut Write) -> io::Result<()>>;

pub trait Renderable {
    fn render(&self, writer: &mut Write) -> io::Result<()>;
}

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
        // let strings: Vec<String> = fs::read_dir(".")?
        //     .filter_map(|r| r.ok())
        //     .map(|e| e.file_name())
        //     .map(|s| s.into_string())
        //     .filter_map(|r| r.ok())
        //     .collect();

        // Ok(State(strings))
        Ok(State(Vec::new()))
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
