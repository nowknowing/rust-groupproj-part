fn main() {
    let mut x = 0;
    let mut y = &mut x;
    //let mut z = &mut y;

    let result = can_change(&mut *y,  &mut y);
    println("iis it 5? : {}", result);
}

fn can_change(alt_ptr_to_content : &mut i32, ptr_to_ptr : &mut &mut i32) -> i32 {
    *alt_ptr_to_content = 5;
    **ptr_to_ptr = 13;
    return *alt_ptr_to_content;

}

