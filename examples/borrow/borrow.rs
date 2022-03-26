fn main() {
    let s1 = String::from("hello");

    let longer_than_three = calculate_length(&s1);

    println!("The length of '{}' is longer than 3 : {}.", s1, longer_than_three);

    let one = 1;
    let take_one = one;
    println!("takeOne : {} borrowed from one: {}.", take_one, one);
}

fn calculate_length(s: &String) -> bool {
    s.len() > 3
}