fn main() {
    let mut x = String::from("hello");
    let y = &x;
    let z = &x;
    println("y: {}", y);
    println("z: {}", z);

    x.push_str(", world");
    println("x: {}", x);

}