fn main() {
    fn longest(x: &'static str, y: &'static str) -> &'static str {
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