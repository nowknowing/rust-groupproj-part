fn main() {
    let ref_to_nth = dangle();
}
fn dangle() -> &str {
    let result = string_from("really long string");
    as_str(result)
}
