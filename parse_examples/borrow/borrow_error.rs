fn main() {
    let mut s = StringFrom("hello");
    let s1 = &mut s;
    push_to_str(&mut s);
    println("{}", s1);
}

fn push_to_str(input: &mut String) -> &String {
    input.push_str(", world");


    input
}