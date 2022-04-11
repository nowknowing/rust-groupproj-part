fn main() {
    let x = plus_one(5);

    println("The value of x is: {}", x);
}

fn plus_one(x: i64) -> i64 {
    x + 1;
}