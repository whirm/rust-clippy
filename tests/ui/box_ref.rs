#![warn(clippy::box_ref)]

fn main() {}

fn string_slice_is_ok_to_box() {
    let the_slice = "";
    let the_ok = Box::new(&the_slice);
}

fn reference_to_a_literal_is_redundant_to_box() {
    let a = 1u32;

    let b = Box::new(&a);
}

fn type_annotations_of_a_boxed_int_is_redundant_to_box() {
    let a = 1u32;

    #[allow(clippy::box_ref)]
    let b = Box::new(&a);

    let c: Box<&u32> = b;
}

// TODO if I remember how to do into() without the type annotation
// fn four() {
//     let a = 1u32;

//     #[allow(clippy::box_ref)]
//     let b = Box::new(&a);

//     #[allow(clippy::box_ref)]
//     let c: Box<&u32> = b;

//     let d = (&a).into();
// }
