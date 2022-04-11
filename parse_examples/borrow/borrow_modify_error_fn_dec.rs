fn main() {
    let s = string_from("hello");

    change(&mut s); // &s also the same error. // i.e. method dec check comes before application check
}

fn change(some_string: &String) {
    push_str(some_string, ", world");
}