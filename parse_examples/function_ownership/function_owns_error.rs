fn main() {
    fn longest<'a>(x: &str, y: &str) -> &'a str {
        let result = StringFrom("really long string");
        AsStr(result)
    }
}
/*
error[E0515]: cannot return reference to local variable `result`

   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
   */