fn main() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("correct");
    let result;
    {
        let string2 = String::from("wrong");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

/*
error[E0597]: `string2` does not live long enough

  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
  */