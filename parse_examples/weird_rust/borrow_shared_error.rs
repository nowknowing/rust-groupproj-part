fn main() {
    let mut x = 0;
    let mut y = &mut x;
    let z = &mut *y;

    *z = 3;
    *y = 5; // all above y popped. y not same tag so no error here.
    println("{}", z); // z is gone. popped alr
    //let f = take_both_print_y( y); // unlike borrow_simple_error.rs
    // no use of z after use of y allowed.
    //println("{}", f);


}
/*
fn take_both_print_y(input_y : &mut String) -> & String {//, input_z : &mut String) {
    println("through fn y :{}", input_y);
    let pt_to_input = & input_y;
    return *pt_to_input;
}
*/

/*
error[E0506]: cannot assign to `*y` because it is borrowed
 --> borrow_shared_error.rs:7:5
  |
4 |     let z = &mut *y;
  |             ------- borrow of `*y` occurs here
...
7 |     *y = 5;
  |     ^^^^^^ assignment to borrowed `*y` occurs here
8 |     println("{}", z);
  |                    - borrow later used here

error: aborting due to previous error; 1 warning emitted
*/