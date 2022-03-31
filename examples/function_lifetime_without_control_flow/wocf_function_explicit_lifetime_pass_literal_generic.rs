fn main() {
    fn first<'a>(x: &'a str, y: &'a str) -> &'a str {
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