fn main() {
    let mut x = 0;
    let y = &mut x;
    let z = &mut x;
    *y = 2; // perfectly fine up till here. i.e. without it's use in next line it's accepted.
    //println!("{}", z); // or println!("{}", x); or println!("{}", z); just one, then allowed

}