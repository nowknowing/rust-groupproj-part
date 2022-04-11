fn main() {
    let mut s = string_from("hello");
    let s1 = &mut s;
    push_to_str(&mut s);
    println("{}", s1);
}

fn push_to_str(input: &mut String) -> &String {
    push_str(input, ", world");


    input
}