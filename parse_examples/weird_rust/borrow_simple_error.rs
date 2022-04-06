fn main() {
    let mut x = 0;
    let y = &mut x;
    let z = &mut x;
    //println("{}", z); // perfectly fine up till here. i.e. without it's use in next line it's accepted.
    println("{}", y); // or println("{}", x); or println("{}", z); just one, then allowed

}
// lifetime of a later coming pointer cannot be shorter than the former.??
//  instead of popping z on use of y, we observe rust disallows altogether!! why??? 
// so does rust want to check on use or check on declaration??


/*
error[E0499]: cannot borrow `x` as mutable more than once at a time
 --> borrow_simple_error.rs:4:13
  |
3 |     let y = &mut x;
  |             ------ first mutable borrow occurs here
4 |     let z = &mut x;// perfectly fine up till here. i.e. without it's use in next line it's accepted.
  |             ^^^^^^ second mutable borrow occurs here
5 |     println("{}", z);
6 |     println("{}", y); // or println("{}", x); or println("{}", z); just one, then allowed
  |                    - first borrow later used here

error: aborting due to previous error
*/