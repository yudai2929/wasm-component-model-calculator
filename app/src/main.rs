mod bindings;

use bindings::component::calculator::calc::Op;

use crate::bindings::component::calculator::calc::calc;

fn main() {
    let a = 2;
    let b = 1;

    let add_result = calc(Op::Add, a, b);
    println!("Add result: {} + {} = {}", a, b, add_result);

    let sub_result = calc(Op::Sub, a, b);
    println!("Sub result: {} - {} = {}", a, b, sub_result);
}
