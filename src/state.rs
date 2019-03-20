use crate::printer::*;
use std::{
    collections::HashMap,
    io::{
        self,
        Stdout
    }
};

type ID = usize;
type Print = Box<Fn(&mut Printer) -> io::Result<()>>;

pub trait Renderable {
    fn render(&self, printer: &mut Printer) -> io::Result<()>;
}

pub struct State {
    items: Vec<Box<dyn Renderable>>,
    before: HashMap<ID, Print>,
    after: HashMap<ID, Print>,
}

impl State {
    pub fn new() -> State {
        State {
            items:      Vec::new(),
            before: HashMap::new(),
            after:  HashMap::new(),
        }
    }

    pub fn before(&mut self, id: ID, func: Print) {
        self.before.insert(id, func);
    }

    pub fn after(&mut self, id: ID, func: Print) {
        self.after.insert(id, func);
    }
    pub fn push(&mut self, item: Box<dyn Renderable>) -> ID {
        let items = &mut self.items;
        let index = items.len();
        items.push(item);
        index
    }
}

impl Renderable for State {
    fn render(&self, printer: &mut Printer) -> io::Result<()> {

        let State{ ref items, ref before, ref after } = self;

        for (id, item) in items.iter().enumerate() {
            if let Some(before) = before.get(&id) {
                before(printer)?;
            }

            item.render(printer)?;

            if let Some(after) = after.get(&id) {
                after(printer)?;
            }

        }
        Ok(())
    }

}
