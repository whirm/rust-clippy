#![warn(clippy::box_ref)]

fn main() {
    let ok1 = "";
    let ok2 = Box::new(&ok1);

    let a = 1u32;

    let b = Box::new(&a);
    //~^ box_ref

    let c: Box<&u32> = b;
    //~^ box_ref

    let d: Box<&u32> = (&a).into();
    //~^ box_ref
}
