fn main() {
    let mut x = 0;
    let y = &mut x;
    println!("{}", y);

    let z = &mut x;
    println!("{}", z);
    
    println!("{}", x); // or println!("{}", y); GIVE ERROR INSTEAD.
}