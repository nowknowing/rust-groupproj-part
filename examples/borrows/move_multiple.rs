fn main() {
    let s = String::from("hello");

    let r1 = s;
    let r2 = r1;

    println!("{}", r2);
}

