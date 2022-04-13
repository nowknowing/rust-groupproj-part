fn main() {

    let mut x = 0;
    let mut y = &mut x;
    let mut z = &mut y;
    let mut a = &mut z;
    /*
    let mut alt_y = &mut x;
    let mut alt_z = &mut y;

    z = &mut alt_y;
    // *a = &mut alt_y;
    //let mut f = &mut a;
   println!("accessing through a {}", a);
   */

   let mut y_alt = &mut (*y);
   z = &mut y_alt;
   println!("{}" , z);

}