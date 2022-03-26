fn main() {
    let s = String::from("hello");

    change(&mut s); // &s also the same error. // i.e. method dec check comes before application check
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}