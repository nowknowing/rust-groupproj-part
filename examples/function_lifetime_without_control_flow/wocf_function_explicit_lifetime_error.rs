fn main() {
    fn first<'a>(x: &'a str, y: &'a str) -> &'a str {
        let second = y;
        println!("second is {}", second);
        x
    }
    
    let string1 = String::from("correct");
    let result;
    {
        let string2 = String::from("wrong");
        result = first(string1.as_str(), string2.as_str());
    }
    println!("The first string is {}", result);
}

/*
error[E0597]: `string2` does not live long enough
  --> wocf_function_explicit_lifetime_error.rs:12:42
   |
12 |         result = first(string1.as_str(), string2.as_str());
   |                                          ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
13 |     }
   |     - `string2` dropped here while still borrowed
14 |     println!("The first string is {}", result);
   |                                        ------ borrow later used here

error: aborting due to previous error
  */