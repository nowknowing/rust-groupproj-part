fn main() {
    // addition
    let sum = 5.0 + 10;

    // subtraction
    let difference = 95 - 4.3;

    // multiplication
    let product = 4.0 * 30;

    // division
    let quotient = 56 / 32.2;
    let floored = 2 / 3.0; // Results in 0

    // remainder
    let remainder = 43 % 5.0;

    println!("{} {} {} {} {} {}", sum, difference, product, quotient, floored, remainder);
}

/*
error[E0277]: cannot add an integer to a float
 --> unannotated_numeric_types_error.rs:3:19
  |
3 |     let sum = 5.0 + 10;
  |                   ^ no implementation for `{float} + {integer}`
  |
  = help: the trait `Add<{integer}>` is not implemented for `{float}`

error[E0277]: cannot subtract `{float}` from `{integer}`
 --> unannotated_numeric_types_error.rs:6:25
  |
6 |     let difference = 95 - 4.3;
  |                         ^ no implementation for `{integer} - {float}`
  |
  = help: the trait `Sub<{float}>` is not implemented for `{integer}`

error[E0277]: cannot multiply `{float}` by `{integer}`
 --> unannotated_numeric_types_error.rs:9:23
  |
9 |     let product = 4.0 * 30;
  |                       ^ no implementation for `{float} * {integer}`
  |
  = help: the trait `Mul<{integer}>` is not implemented for `{float}`

error[E0277]: cannot divide `{integer}` by `{float}`
  --> unannotated_numeric_types_error.rs:12:23
   |
12 |     let quotient = 56 / 32.2;
   |                       ^ no implementation for `{integer} / {float}`
   |
   = help: the trait `Div<{float}>` is not implemented for `{integer}`

error[E0277]: cannot divide `{integer}` by `{float}`
  --> unannotated_numeric_types_error.rs:13:21
   |
13 |     let floored = 2 / 3.0; // Results in 0
   |                     ^ no implementation for `{integer} / {float}`
   |
   = help: the trait `Div<{float}>` is not implemented for `{integer}`

error[E0277]: cannot mod `{integer}` by `{float}`
  --> unannotated_numeric_types_error.rs:16:24
   |
16 |     let remainder = 43 % 5.0;
   |                        ^ no implementation for `{integer} % {float}`
   |
   = help: the trait `Rem<{float}>` is not implemented for `{integer}`

error: aborting due to 6 previous errors
*/