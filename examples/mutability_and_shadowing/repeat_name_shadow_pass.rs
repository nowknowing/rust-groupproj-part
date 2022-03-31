fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // different types
    let spaces = "   ";
    println!("There are spaces {} here.", spaces);
    let spaces = spaces.len();
    println!("The space is of length {}.", spaces);
}
//12
//6