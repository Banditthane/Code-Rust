fn show_info(name: &str, age: u32) {
    println!("\nMy name is {}.", name);
    println!("I am {} years old.", age);
}

// part: 2
fn aad(a: i32, b: i32) {
    println!("\nAddition: {}", a + b);
}

// part: 3
fn multiplay(a: i32, b: i32) {
    println!("\nMultiplication: {}", a * b);
}

// part: เพิ่มเติม
fn show_score(score: u32) {
    println!("\nMy score is {}.", score);
}
fn main() {
    show_info("Auck", 22);
    aad(10, 20);
    multiplay(10, 20);
    show_score(99);
}
