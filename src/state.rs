use std::{
    collections::HashMap,
    io::{
        self,
        Stdout,
        Write
    }
};

type ID = usize;
type Render = Box<Fn(&mut Write) -> io::Result<()>>;

pub trait Renderable {
    fn render(&self, writer: &mut Write) -> io::Result<()>;
}

pub struct State {
    items: Vec<Box<dyn Renderable>>,
    before: HashMap<ID, Render>,
    after: HashMap<ID, Render>,
}

impl State {
    pub fn new() -> State {
        State {
            items:      Vec::new(),
            before: HashMap::new(),
            after:  HashMap::new(),
        }
    }

    pub fn before(&mut self, id: ID, func: Render) {
        self.before.insert(id, func);
    }

    pub fn after(&mut self, id: ID, func: Render) {
        self.after.insert(id, func);
    }

    pub fn clear(&mut self, id: ID) -> (Option<Render>, Option<Render>) {
        (self.before.remove(&id), self.after.remove(&id))
    }

    pub fn push(&mut self, item: Box<dyn Renderable>) -> ID {
        let items = &mut self.items;
        let index = items.len();
        items.push(item);
        index
    }
}

impl Renderable for State {
    fn render(&self, writer: &mut Write) -> io::Result<()> {

        let State{ ref items, ref before, ref after } = self;

        for (id, item) in items.iter().enumerate() {
            if let Some(before) = before.get(&id) {
                before(writer)?;
            }

            item.render(writer)?;

            if let Some(after) = after.get(&id) {
                after(writer)?;
            }

        }
        Ok(())
    }

}
