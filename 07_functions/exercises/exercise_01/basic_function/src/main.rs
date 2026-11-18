fn greet() {
    println!("Hello, Rust!");
}

fn show_name(name: &str) {
    println!("\nMy name is {}.", name);
}
fn show_age(age: u32) {
    println!("I am {} years old.", age);
}

// เพิ่มเติม
fn show_country() {
    println!("\nI am from Thailand.");
}
fn show_language() {
    println!("I speak Thai.");
}

fn main() {
    let name = "Auck";
    let age = 22;

    greet();
    greet();

    show_name(name);
    show_age(age);
    show_country();
    show_language();
}
