/// Topic: Immutable variables.
pub(crate) fn variables_are_immutable() {
    let x = 5;
    println!("x={x}");

    // Uncomment the line below. It will generate a compile error.
    // x = 6;
}

pub(crate) fn variables_can_be_mutable() {
    let mut x = 5;
    println!("x={x}");
    x = 6;
    println!("x={x}");
}