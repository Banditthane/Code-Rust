//แบบ basic
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
// แบบย่อ
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: i32, b: i32) -> f64 {
    a as f64 / b as f64
}

fn main() {
    println!("Addition: {}", add(20, 5));
    println!("Subtraction: {}", subtract(20, 5));
    println!("Multiplication: {}", multiply(20, 5));
    println!("Division: {}", divide(20, 5));
}
