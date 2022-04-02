fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("mutable r1 {}", r1); // r4 : & String

    let r2 = &mut (*r1);

    println!("mutable r2 {}", r2); // r4 : & String
    //push_to_str(r1);
    let r3 = push_to_str(r2); // mutate r2
   // println!("mutable r2 after mutation of r2 :{}", r2);

    println!("mutable r1 after mutation of r2 :{}", r1);
    //println!("mutable r2 after mutation of r2 :{}", r2);

}

fn push_to_str(input: &mut String) -> &mut String {
    input.push_str(", world");
    input
}