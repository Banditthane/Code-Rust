fn main() {
    //ส่วนที่ 1: แสดงตัวเลข
    println!("ส่วนที่ 1: แสดงตัวเลข 1 ถึง 20");

    for number in 1..=20 {
        println!("{}", number);
    }

    //ส่วนที่ 2: แสดงตัวเลขคู่
    println!("\nส่วนที่ 2: แสดงตัวเลขคู่ 1 ถึง 20");

    for number in 1..=20 {
        if number % 2 == 0 {
            println!("{}", number);
        }
    }

    //ส่วนที่ 3: วนซ้ำกับ Array
    println!("\nส่วนที่ 3: วนซ้ำกับ Array");

    let numbers = [10, 20, 30, 40, 50];

    for number in numbers {
        println!("{}", number);
    }

    //เพิ่มเติม
    println!("\nโจทย์เพิ่มเติม: แสดงตัวเลขจากมากไปน้อย");

    for number in (1..=20).rev() {
        println!("{}", number);
    }
}
