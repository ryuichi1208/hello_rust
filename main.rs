fn main() {

    let mut x;
    x = 6;
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is: {}", x);

    let x:i32 = 6;
    println!("The value of x is: {}", x);

    let y:u32 = 6;
    println!("The value of x is: {}", y);

    println!("The value of x is: {}", add(2, 3));
}

fn add(x:i32, y:i32) -> i32 {
    x + y
}
