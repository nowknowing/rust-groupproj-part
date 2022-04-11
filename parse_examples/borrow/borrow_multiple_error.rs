fn main() {
    //only once of mutable is allowed. first mutable later used.
    let mut s = string_from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println("{}, {}", r1, r2);

    //if immutably borrowed, no mutable borrow allowed. immutable later used.
    let mut m  = string_from("world");
    let m1 = & m;
    let m2 = &mut m;
    println("{} {}", m1, m2);

    // cannot borrow mutable after immutable.
    let mut n = string_from("hey");
    let n1 = &mut n;
    let n2 = & n;
    println("{} {}", n1, n2);
}
/*

error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_multiple_error.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |     println("{}, {}", r1, r2);
  |                        -- first borrow later used here

error[E0502]: cannot borrow `m` as mutable because it is also borrowed as immutable
  --> borrow_multiple_error.rs:11:14
   |
10 |     let m1 = & m;
   |              --- immutable borrow occurs here
11 |     let m2 = &mut m;
   |              ^^^^^^ mutable borrow occurs here
12 |     println("{} {}", m1, m2);
   |                       -- immutable borrow later used here

error[E0502]: cannot borrow `n` as immutable because it is also borrowed as mutable
  --> borrow_multiple_error.rs:17:14
   |
16 |     let n1 = &mut n;
   |              ------ mutable borrow occurs here
17 |     let n2 = & n;
   |              ^^^ immutable borrow occurs here
18 |     println("{} {}", n1, n2);
   |                       -- mutable borrow later used here

error: aborting due to 3 previous errors

*/