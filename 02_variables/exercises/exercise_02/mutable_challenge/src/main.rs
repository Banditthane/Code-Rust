fn main() {
    let mut age = 20;
    let mut score = 50;
    let mut level = 1;

    println!("Before Update");
    println!("Age: {age}");
    println!("Score: {score}");
    println!("Level: {level}");

    age = 21;
    score = 100;
    level = 2;

    println!("After Update");
    println!("Age: {age}");
    println!("Score: {score}");
    println!("Level: {level}");
}