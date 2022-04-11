fn main() {
    let s = string_from("hello");

    change(&s);
}

fn change(some_string: &mut String) {
    push_str(some_string, ", world");
}