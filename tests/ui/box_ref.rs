#![warn(clippy::box_ref)]

fn main() {}

fn string_slice_is_ok_to_box() {
    let the_slice = "";
    let the_ok = Box::new(&the_slice);
}

fn reference_to_a_literal_is_redundant_to_box() {
    let a = 1u32;

    let b = Box::new(&a);
    //~^ box_ref
}

fn type_annotations_of_a_boxed_int_is_redundant_to_box() {
    let a = 1u32;

    #[allow(clippy::box_ref)]
    let b = Box::new(&a);

    let c: Box<&u32> = b;
    //~^ box_ref
}

fn box_from_thin_ref_is_redundant_to_box() {
    let a = 1u32;

    let b = Box::from(&a);
    //~^ box_ref
}

// TODO:
// From https://github.com/rust-lang/rust-clippy/issues/2394
// Box<&T>
// Box::new(SomeT) where sizeof::<T>() <= sizeof::<usize>()
// unless there are Box::into_raw calls within the function
// Rc<Box<T>>
// Rc<Rc<T>>
// Rc<&T>
