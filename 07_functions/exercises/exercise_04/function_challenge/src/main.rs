// ส่วนที่ 1: แสดงข้อมูลคะแนน
fn show_scores(subject: &str, score: u32) {
    println!("{}: {}", subject, score);
}

// ส่วนที่ 2: คำนวณคะแนนรวม
fn calculate_total(math: u32, science: u32, english: u32) -> u32 {
    math + science + english
}

// ส่วนที่ 3: คำนวณคะแนนเฉลี่ย
fn calculate_average(total: u32, subjects: u32) -> f64 {
    if subjects == 0 {
        return 0.0;
    }

    total as f64 / subjects as f64
}

// ส่วนที่ 4: ตรวจสอบผลสอบ
fn is_passed(score: u32) -> bool {
    score >= 50
}

// โจทย์เพิ่มเติม: คำนวณเกรด
fn get_grade(score: u32) -> &'static str {
    if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else if score >= 60 {
        "C"
    } else if score >= 50 {
        "D"
    } else {
        "F"
    }
}

// ส่วนที่ 5: เรียกใช้งาน Function
fn main() {
    // กำหนดคะแนนเริ่มต้น
    let math: u32 = 80;
    let science: u32 = 70;
    let english: u32 = 90;

    // แสดงคะแนนแต่ละวิชา
    show_scores("Mathematics", math);
    show_scores("Science", science);
    show_scores("English", english);

    println!();

    // คำนวณคะแนนรวม
    let total = calculate_total(math, science, english);
    println!("Total Score: {}", total);

    // คำนวณคะแนนเฉลี่ย
    let average = calculate_average(total, 3);
    println!("Average Score: {:.0}", average);

    println!();

    // ตรวจสอบผลสอบ
    println!("Mathematics Passed: {}", is_passed(math));
    println!("Science Passed: {}", is_passed(science));
    println!("English Passed: {}", is_passed(english));

    println!();

    // แสดงเกรด
    println!("Mathematics Grade: {}", get_grade(math));
    println!("Science Grade: {}", get_grade(science));
    println!("English Grade: {}", get_grade(english));
}
