fn main() {
    let mut x = string_from("hello");
    let y = &x;
    let z = &x;
    println("y: {}", y);
    println("z: {}", z);

    push_str(x, ", world");
    println("x: {}", x);

}