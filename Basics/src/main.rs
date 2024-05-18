fn main() {
    add();
    sub();
}

fn add() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Sum of {} and {} is {}", a, b, c);
}

fn sub() {
    let a:i32 = 20;
    let b:i64 = 10;
    let c:i32 = a-b as i32;
    println!("Difference of {} and {} is {}", a, b, c);
    
}
