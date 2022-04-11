fn main() {
    let mut x : i64 = 5;
    println("The value of x is: {}", x);
    x = true;
    println("The value of x is: {}", x);
}
/*
error[E0658]: type ascription is experimental
 --> typed_mutability_error.rs:4:5
  |
4 |     x : bool = true;
  |     ^^^^^^^^
  |
  = note: see issue #23416 <https://github.com/rust-lang/rust/issues/23416> for more information

error[E0308]: mismatched types
 --> typed_mutability_error.rs:4:5
  |
4 |     x : bool = true;
  |     ^ expected `bool`, found `u32`
*/