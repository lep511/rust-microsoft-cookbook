mod mathlib;

use mathlib::{math};

fn main() {
    println!("{}", math::sin(45.0));
    println!("{}", math::cos(45.0));
    println!("{}", math::tan(45.0));
    println!("{:?}", math::add((1.0, 2.0), (3.0, 4.0)));
}