fn main() {
    add();
    sub();
    multi();
    div();
    rem();
}

fn add() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Sum of {} and {} is {}", a, b, c);
}

fn sub() {
    let a: i32 = 20;
    let b: i64 = 10;
    let c: i32 = a - (b as i32);
    println!("Difference of {} and {} is {}", a, b, c);
}

fn multi() {
    let a: f32 = 10.2;
    let b: f32 = 12.2;
    let c = a * b;
    println!("Product of {} and {} is {}", a, b, c);
}

fn div() {
    let a = 500 as f64;
    let b = 10.5;
    let c = a / (b as f64);
    println!("Division of {} and {} is {}", a, b, c);
}

fn rem() {
    let a = 1343;
    let b = 10;
    let c = a % b;
    println!("Remainder of {} and {} is {}", a, b, c);
}
