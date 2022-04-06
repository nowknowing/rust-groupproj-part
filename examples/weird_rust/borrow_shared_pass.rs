fn main() {
    let mut x = String::from("hello");
    let y = &mut x;
    let z = &mut *y;

    println!("z: {}", z);
    my_print(z);

    println!("y: {}", y); // unlike borrow_simple_error.rs
    // no use of z after use of y allowed.
    my_print(y);




    let mut l = 13;
    let a = &mut l;
    let b = &mut *a;  // fails if &mut l;
    *b = 3;
    *a = 4; // pops b
}


fn my_print(input: &mut String) {
println!("through my_print: {}", input);
}
//pop happens, instead of disallow altogether in borrow_simple_error.rs
