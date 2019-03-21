use crate::{
    state::*,
    types::*,
};

use std::{
    collections::HashMap,
    io::{
        self,
        Write
    },
    rc::Rc
};

pub struct Renderer {
    state: Rc<State>,
    before: HashMap<ID, Render>,
    after: HashMap<ID, Render>
}

impl Renderer {
    pub fn new(state: Rc<State>) -> Renderer {
        Renderer {
            state,
            before: HashMap::new(),
            after:  HashMap::new()
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

    pub fn swap(&mut self, state: Rc<State>) {
        self.state = state;
    }
}

impl Renderable for Renderer {
    fn render(&self, writer: &mut Write) -> io::Result<()> {
        let Renderer{ ref before, ref after, ref state } = self;

        for (id, item) in state.iter().enumerate() {
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
