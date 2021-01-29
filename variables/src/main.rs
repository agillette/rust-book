fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("The max points is: {}", MAX_POINTS);
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {}", b);
}
