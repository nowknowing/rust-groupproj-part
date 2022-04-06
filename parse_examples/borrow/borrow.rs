fn main() {
    let s1 = StringFrom("hello");

    let longer_than_three = calculate_length(&s1);

    println("The length of '{}' is longer than 3 : {}.", s1, longer_than_three);

    let one = 1;
    let take_one = one;
    println("takeOne : {} copy not borrow from one: {}.", take_one, one);

    let s2 = s1; // move s1 to 2
    println("The borrowed string is '{}'.", s2);

    takes_and_gives_back(s2);
    println("The borrowed string is '{}'.", s2);

}

fn calculate_length(s: &String) -> bool {
    Len(s) > 3
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

a_string  // a_string is returned and moves out to the calling function
}