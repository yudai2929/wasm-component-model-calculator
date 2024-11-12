#[allow(warnings)]
mod bindings;

use crate::bindings::exports::component::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(x: u32, y: u32) -> u32 {
        x + y
    }
}

bindings::export!(Component with_types_in bindings);
