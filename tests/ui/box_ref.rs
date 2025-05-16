#![warn(clippy::box_ref)]

fn main() {
    let a = "";

    let b = Box::new(a);
    //~^ box_ref

    let c: Box<&str> = b;
    //~^ box_ref
}
