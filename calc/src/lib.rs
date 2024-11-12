#[allow(warnings)]
mod bindings;

use bindings::component::subtractor::sub::sub;
use bindings::exports::component::calculator::calc::Op;

use crate::bindings::component::adder::add::add;
use crate::bindings::exports::component::calculator::calc::Guest;

struct Component;

impl Guest for Component {
    fn calc(op: Op, x: u32, y: u32) -> u32 {
        match op {
            // Calling a function implemented in Rust
            Op::Add => add(x, y),
            // Calling a function implemented in Go
            Op::Sub => sub(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
