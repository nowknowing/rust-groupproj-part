fn main() {
    let y = {
        let x = 3;
        x + 1;
    };

    //lifetime ends in parenthesis
    {
        let z = 3;
        z + 2;
    }
    z + 3;

    println!("The value of y is: {}", y);
}