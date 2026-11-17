fn main() {
    //ส่วนที่ 1: แสดงตัวเลข
    println!("ส่วนที่ 1: แสดงตัวเลข 1 ถึง 30");

    for number in 1..=30 {
        println!("{}", number);
    }

    //ส่วนที่ 2: จำแนกเลขคู่และเลขคี่
    println!("\nส่วนที่ 2: จำแนกเลขคู่และเลขคี่ 1 ถึง 30");

    for number in 1..=30 {
        if number % 2 == 0 {
            println!("เลขคู่: {}", number);
        } else {
            println!("เลขคี่: {}", number);
        }
    }

    //ส่วนที่ 3: คำนวณผลรวม
    println!("\nส่วนที่ 3: คำนวณผลรวมของตัวเลข 1 ถึง 30");

    let mut total = 0;

    for number in 1..=30 {
        total += number;
    }
    println!("ผลรวม: {}", total);

    //ส่วนที่ 4: ฝึกใช้ break และ continue
    println!("\nส่วนที่ 4: ฝึกใช้ break และ continue");

    let mut count = 0;
    loop {
        count += 1;
        if count == 10 {
            break;
        }
        if count % 2 == 0 {
            continue;
        }
        println!("Count: {}", count);
    }

    //ส่วนที่ โจทย์เพิ่มเติม

    // ข้อ 1: เปลี่ยนช่วงตัวเลขจาก 1-30 เป็น 1-100
    println!("\nส่วนที่ โจทย์เพิ่มเติม: แสดงตัวเลข 1 ถึง 100");

    for number in 1..=100 {
        println!("{}", number);
    }

    //2. แสดงผลรวมของเลขคู่
    println!("\nส่วนที่ โจทย์เพิ่มเติม: แสดงตัวเลข 1 ถึง 100 และจำแนกเลขคู่และเลขคี่");

    let mut even_total = 0;
    let mut odd_total = 0;

    for number in 1..=100 {
        if number % 2 == 0 {
            //println!("เลขคู่: {}", number);
            even_total += number;
        } else if number % 2 != 0 {
            //println!("เลขคี่: {}", number);
            odd_total += number;
        }
    }
    println!("ผลรวมของเลขคู่: {}", even_total);
    println!("ผลรวมของเลขคี่: {}", odd_total);

    // try
    let mut number = 1;

    while number <= 30 {
        println!("{}", number);
        number += 1;
    }
}
