fn example1 (x: & mut i32 , y: & mut i32 ) -> i32 {
    *x = 42;
    *y = 13;
    *x // Has to read 42 , because x and y cannot alias !
}

fn example2 (x: & i32, y: & i32) -> i32 {
    *x + *y
}
fn main() {
    let mut x = 1;
    let mut y = 2;
    x = example1(&mut x, &mut y);
    let sum = example2(&x, &y);
    println!("x is changed to 42: {}", x);
    println!("sum is now 55: {}", sum);
}