fn main() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = "correct";
    let result;
    {
        let string2 = "wrong";
        result = longest(string1, string2);
    }
    println!("The longest string is {}", result);
}