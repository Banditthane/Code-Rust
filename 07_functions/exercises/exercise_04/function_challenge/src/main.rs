fn show_scores(subject: &str, score: u32) {
    println!("{}: {}", subject, score);
}

fn calculate_total(math: u32, science: u32, english: u32) -> u32 {
    math + science + english
}

fn calculate_average(total: u32, subjects: u32) -> f64 {
    if subjects == 0 {
        return 0.0;
    }

    total as f64 / subjects as f64
}

fn is_passed(score: u32) -> bool {
    score >= 50
}

fn get_grade(score: u32) -> &str {
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

fn main() {
    // Student scores
    let math = 80;
    let science = 70;
    let english = 90;

    // Part 1: Show scores
    show_scores("Mathematics", math);
    show_scores("Science", science);
    show_scores("English", english);

    // Part 2: Calculate total
    let total = calculate_total(math, science, english);
    println!("\nTotal Score: {}", total);

    // Part 3: Calculate average
    let average = calculate_average(total, 3);
    println!("Average Score: {}", average);

    // Part 4: Check pass or fail
    println!("\nMathematics Passed: {}", is_passed(math));
    println!("Science Passed: {}", is_passed(science));
    println!("English Passed: {}", is_passed(english));

    // Additional: Show grades
    println!("\nMathematics Grade: {}", get_grade(math));
    println!("Science Grade: {}", get_grade(science));
    println!("English Grade: {}", get_grade(english));
}
