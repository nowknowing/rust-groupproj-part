fn main() {
    let mut s = StringFrom("hello");
    let r1 = &mut s;
    println("mutable r1 {}", r1); // r4 : & String

    let r2 = &mut s;
    println("mutable r2 {}", r2); // r4 : & String
 //   push_to_str(r2); // mutate r2
 let mut moved = s;
    println("mutable moved :{}", moved);
    //let mut r3 = makestr();
    //println("mutable r3 {}", r3); // r4 : & String
    //push_to_str(&mut r3);
    //println("mutable r3 {}", r3); // r4 : & String

}


fn push_to_str(input: &mut String) -> &String {
    input.push_str(", world");
    let my_mut : & String = input; 

    my_mut
}


fn makestr() -> String {
    let mut mine = StringFrom("foo");
    mine
}