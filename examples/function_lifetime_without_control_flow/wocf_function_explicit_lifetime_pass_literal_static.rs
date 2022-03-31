fn main() {
    fn first(x: &'static str, y: &'static str) -> &'static str {
        let second = y;
        println!("second is {}", second);
        x
    }
    
    let string1 = "first";
    let result;
    {
        let string2 = "second";
        result = first(string1, string2);
    }
    println!("The first string is {}", result);
}