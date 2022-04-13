fn main() {
    let mut mys = String::from("hello");
    let myy = &mut mys;
    let myz = *myy;
}