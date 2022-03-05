fn main() {
    let s1 = String::from("hello");

    let longer_than_three = calculate_length(&s1);

    println!("The length of '{}' is longer than 3 : {}.", s1, longer_than_three);
}

fn calculate_length(s: &String) -> bool {
    s.len() > 3
}