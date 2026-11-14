fn main() {
    let app_name = "Rust Learning";

    let mut age = 20;
    let mut score = 50;

    const COUNTRY: &str = "Thailand";
    const MAX_LEVEL: u32 = 10;

    println!("Application: {app_name}");
    println!("Country: {COUNTRY}");
    println!("Maximum Level: {MAX_LEVEL}");

    println!("Before Update");
    println!("Age: {age}");
    println!("Score: {score}");

    age = 21;
    score = 100;

    println!("After Update");
    println!("Age: {age}");
    println!("Score: {score}");
}