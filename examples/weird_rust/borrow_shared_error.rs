fn main() {
    let mut x = String::from("hello");
    let y = &mut x;
    let z = &mut (*y);

    take_both_print_y(y, z); // unlike borrow_simple_error.rs

    // no use of z after use of y allowed.
}
fn take_both_print_y(input_y : &mut String, input_z : &mut String) {
    println!("through fn y :{}", input_y);
}


// reject altogether because there is no way to 
// guarantee that there is no use of input_z after use of input_y? 
// why not just translate that info into the fun scope as well????