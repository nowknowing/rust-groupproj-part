fn main() {
    fn first<'a>(x: &'a str, y: &'a str) -> &'a str {
        let second = y;
        println!("second is {}", second);
        x
    }

    let string1 = String::from("first");

    {
        let string2 = String::from("second");
        let result = first(string1.as_str(), string2.as_str());
        println!("The first string is {}", result);
    }
}