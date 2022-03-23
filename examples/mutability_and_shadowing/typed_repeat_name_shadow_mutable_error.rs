fn main() {
    let mut number : u32 = 8;
    println!("Number is : {}.", number);

    number = true;
    println!("Number is {}.", number);
}
/*
error[E0308]: mismatched types
 --> typed_repeat_name_shadow_mutable_error.rs:5:14
  |
2 |     let mut number : u32 = 8;
  |                      --- expected due to this type
...
5 |     number = true;
  |              ^^^^ expected `u32`, found `bool`
  */