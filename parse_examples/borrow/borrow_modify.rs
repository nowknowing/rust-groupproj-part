fn main() {
    let mut s = string_from("hello");

    change(&mut s);
    change(&mut s);
    println("{}", s);
}

fn change(some_string: &mut String) {
    push_str(some_string, ", world");
}
