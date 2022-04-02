fn main() {
    let mut x = 0;
    let mut y = &mut x;
    let z = &mut y;

    **z = 2;
    //1) x = 2; OR print x // can only use x. cannot y, cannot z.
    //2) *y = 2; OR print y// can only use y and x.
    //3) **z = 2; OR print z// can use z, y, x.
    println!("{}", z);

    let mut y_prime = &mut x;
    *y_prime = 3; // not allowed. this is equivalent to use of y.
    println!("{}", y_prime);
}