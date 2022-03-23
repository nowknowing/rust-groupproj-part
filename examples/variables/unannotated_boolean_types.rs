fn main() {
    let t = true;
    let f = false; 
    let tnf = t && f;
    println!("true AND false is false: {}", t && f);
    println!("true OR false is true: {}", t || f);
    println!("NOT true is false: {}", !t);
    println!("NOT false is true: {}", !f);
    println!("true AND false is false: {}", tnf);
}