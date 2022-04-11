fn main() {
    let x : i64 = 5;

    let x : i64 = x + 1;

    {
        let x : i64 = x * 2;
        println("The value of x in the inner scope is: {}", x);
    }

    println("The value of x is: {}", x);

    // different types
    let x : bool = true;
    println("The value is {}.", x);
}
//12
//6