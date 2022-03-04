fn main() {
    //only once of mutable is allowed. first mutable later used.
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);

    //if immutably borrowed, no mutable borrow allowed. immutable later used.
    let mut m  = String::from("world");
    let m1 = & m;
    println!("{}", m1);

    let m2 = &mut m;
    println!("{} {}", m1, m2);

    // cannot borrow mutable after immutable.
    let mut n = String::from("hey");
    let n1 = &mut n;
    println!("{}", n1);

    let n2 = & n;
    println!("{} {}", n1, n2);
}
