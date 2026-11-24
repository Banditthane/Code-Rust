fn show_info(name: &str) {
    println!("Welcome, {}!", name);
}

fn main() {
    //String
    let name: String = String::from("AucK");
    //&str
    let country: &str = "Thailand";
    //String
    let language: String = String::from("Rust");

    println!("Name: {}", name);
    println!("Country: {}", country);
    println!("Language: {}", language);
    show_info(&name);

    // Additional Exercise
    println!("\n--- Additional Exercise ---");

    let mut message: String = String::new();

    //เพิ่มข้อความ
    message.push_str("Hello, ");
    message.push_str(&name);
    message.push_str(" from ");
    //เพิ่มตัวอักษร
    message.push('R');
    message.push_str("ust.");

    println!("{}", message);
    show_info("Rust learner");
}
