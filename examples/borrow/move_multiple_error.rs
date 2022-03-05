fn main() {
    let s = String::from("hello");

    let r1 = s;
    let r2 = s;

    println!("{}, {}", r1, r2);
}
/*error[E0382]: use of moved value: `s`
 --> move_multiple_error.rs:5:14
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |
4 |     let r1 = s;
  |              - value moved here
5 |     let r2 = s;
  |              ^ value used here after move

error: aborting due to previous error
*/
