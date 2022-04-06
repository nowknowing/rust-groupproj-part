fn main() {
    let mut x = 0;
    let mut y = &mut x;
    let mut z = &mut x;
    y = &mut x;
    println("{}", y);
}

