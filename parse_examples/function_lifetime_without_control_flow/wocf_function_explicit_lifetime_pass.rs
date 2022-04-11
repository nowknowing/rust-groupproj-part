fn main() {
    fn first<'a>(x: &'a str, y: &'a str) -> &'a str {
        let second = y;
        println("second is {}", second);
        x
    }

    let string1 = string_from("first");

    {
        let string2 = string_from("second");
        let result = first(as_str(string1), as_str(string2));
        println("The first string is {}", result);
    }
}