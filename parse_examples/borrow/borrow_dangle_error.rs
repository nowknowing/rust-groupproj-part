fn main() {
    let s = string_from("hello");
    let r1 = &s;
    let r2 = &r1;

    let s1 = s;

    println("{}", r2);
}

/*
error[E0505]: cannot move out of `s` because it is borrowed
 --> borrow_dangle_error.rs:6:14
  |
3 |     let r1 = &s;
  |              -- borrow of `s` occurs here
...
6 |     let s1 = s;
  |              ^ move out of `s` occurs here
7 |
8 |     println("{}", r2);
  |                    -- borrow later used here

error: aborting due to previous error; 1 warning emitted
*/