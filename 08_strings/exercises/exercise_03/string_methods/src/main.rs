fn main() {
    let message = String::from("  Hello Rust Programming");

    //Part 1
    println!("Length: {}", message.len());

    //Part 2
    println!("{}", message.contains("Rust"));
    println!("{}", message.contains("Java"));

    //Part 3
    println!("{}", message.trim());

    let new_message = message.replace("Rust", "Java");
    println!("{}",new_message);

    //Part
}
