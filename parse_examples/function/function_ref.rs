fn example1 (x: & mut i64 , y: & mut i64 ) -> i64 {
    *x = 42;
    *y = 13;
    *x 
}

fn example2 (x: & i64, y: & i64) -> i64 {
    x + y
}

fn example3 (x: & i64, y : & i64) -> i64 {
    println("{}, {}", x ,y);
    *x
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    x = example1(&mut x, &mut y);
    println("x is changed to 42: {}", x);
    let sum = example2(&x, &y);
    let larger = example3(&x, &y);
    println("sum is now 55: {}", sum);
    println("larger is : {} than {}", larger, y);  
}