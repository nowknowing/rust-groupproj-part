fn main() {
    let ref_to_nth = dangle();
}
fn dangle() -> &str {
    let result = String::from("really long string");
    result.as_str()
}
