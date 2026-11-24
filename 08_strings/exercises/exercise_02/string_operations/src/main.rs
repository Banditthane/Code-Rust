fn main() {
    let mut message: String = String::new();

    message.push_str("My name is: ");
    message.push_str("Auck");
    message.push('!');

    println!("{}", message);

    // ส่วนที่ 2: ต่อข้อความด้วย +

    let country: &str = "Thailand";
    let country_message = String::from("Country: ") + &country;
    println!("{}", country_message);

    // ทดลองต่อข้อความด้วย format!
    let _country_message2 = format!("Country: {}", country);

    //Part 3
    //let language: &str = "Language: "; // ใช้ไม่ได้
    //let mut language = String::from("Language: ");
    let mut language: String = String::new();

    //println!("{}", language);
    language.push_str("Language: ");
    language.push_str("Rust");
    language.push('!');

    println!("{}", language);

    // ทดลองต่อ String หลายตัวด้วย format!
    let full_message = format!("{} | {} | {}", message, _country_message2, language);

    println!("{}", full_message);
}
