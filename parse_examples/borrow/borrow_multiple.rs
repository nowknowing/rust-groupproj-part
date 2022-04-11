fn main() {
    let mut s = string_from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println("r1: {}, r2: {}", r1, r2);

    let r3 = &mut s; // no problem
    println("r3: {}", r3);
    let r4 = &mut s;
    println("r4: {} ", r4);

    //can borrow. no using of immutable once mutable is borrowed.
    let mut m  = string_from("world");
    let m1 = & m;
    println("immutable m1: {}", m1);
    let m2 = &mut m;
    println("mutable m2: {}", m2);


    // can borrow. no mutating once immutable borrowed.
    let mut n = string_from("hey");
    let n1 = &mut n;
    println("mutable n1: {}", n1);
    let n2 = & n;
    println("immutable n2: {}", n2);

}