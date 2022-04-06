fn main() {
    let s = StringFrom("hello");

    change(&s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}