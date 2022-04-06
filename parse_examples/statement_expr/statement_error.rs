fn main() {
    let x = let y = 6;
}
/*error: expected expression, found statement (`let`)
 --> statement_error.rs:2:13
  |
2 |     let x = let y = 6;
  |             ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are experimental
 --> statement_error.rs:2:13
  |
2 |     let x = let y = 6;
  |             ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
  = help: you can write `matches!(<expr>, <pattern>)` instead of `let <pattern> = <expr>`
  */