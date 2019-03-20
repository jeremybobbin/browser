mod printer;
mod state;
mod line;

use printer::*;
use state::*;
use line::*;
use std::{
    ops::{
        Deref,
        DerefMut
    },
    io::{
        self,
        Write
    }
};

fn main() {
    let mut stdout = io::stdout();
    let mut printer = Printer::new(stdout);
    let l = Box::new(Line::new("foo".to_string()));
    let m = Box::new(Line::new("bang arrr".to_string()));
    let mut state = State::new();
    let id1 = state.push(l);
    let id2 = state.push(m);
    state.before(id1, Box::new(|printer: &mut Printer| {
        printer.red()
    }));
    state.after(id1, Box::new(|printer: &mut Printer| {
        printer.regular()
    }));
    printer.reset().unwrap();
    printer.flush().unwrap();
    state.render(&mut printer).unwrap();
    printer.flush().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
}
