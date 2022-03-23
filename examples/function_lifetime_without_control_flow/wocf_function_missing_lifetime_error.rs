fn main() {
    fn first(x: &str, y: &str) -> &str {
        let second = y;
        println!("second is {}", second);
        x
    }

    let string1 = "first";
    let string2 = "second";

    let result = first(string1, string2);
    println!("The first string is {}", result);
}