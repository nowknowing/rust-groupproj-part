fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{}, {}", r1, r2);

    //can borrow. no using of immutable once mutable is borrowed.
    let mut m  = String::from("world");
    let m1 = & m;
    println!("{}", m1);
    let m2 = &mut m;
    println!("{}", m2);


    // can borrow. no mutating once immutable borrowed.
    let mut n = String::from("hey");
    let n1 = &mut n;
    println!("{}", n1);
    let n2 = & n;
    println!("{}", n2);

}