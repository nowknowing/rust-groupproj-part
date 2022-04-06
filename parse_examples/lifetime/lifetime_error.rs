fn main(){
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println("r: {}", r); //          |
}                         // ---------+

/*
error[E0597]: `x` does not live long enough

   |
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  | 
10 |         println!("r: {}", r);
   |                           - borrow later used here
   */