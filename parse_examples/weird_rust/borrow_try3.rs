fn main() {
    let mut mys = String::from("hello");
    let myy = &mut mys;
    getlen(mys);        
}

fn getlen(input_str : String) -> usize{
    input_str.len()
}