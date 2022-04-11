fn main() {
    let x = 3 / ( 3 - 3);
    let y = 4 / 0;
    println("{} {}", x , y);
}
/*
error: this operation will panic at runtime
 --> divide_by_zero_error.rs:2:13
  |
2 |     let x = 3 / ( 3 - 3);
  |             ^^^^^^^^^^^^ attempt to divide `3_i64` by zero
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
 --> divide_by_zero_error.rs:3:13
  |
3 |     let y = 4 / 0;
  |             ^^^^^ attempt to divide `4_i64` by zero
  */