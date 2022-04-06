fn main() {
    let ref_to_nth = dangle();
}
fn dangle() -> &str {
    let result = StringFrom("really long string");
    AsStr(result)
}
