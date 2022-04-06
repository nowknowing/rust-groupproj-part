fn main() {
    let s = StringFrom("hello");

    let r1 = s;
    let r2 = r1;

    println("{}", r2);
}

