fn main() {
    let math_score: u32 = 80;
    let science_score: u32 = 70;
    let english_score: u32 = 90;
    let bonus: u32 = 10;
    let passing_score: u32 = 50;

    //Part 1 Atrithmetic
    let total_score = math_score + science_score + english_score;
    let average_score = total_score / 3;
    let total_with_bonus = total_score + bonus;

    println!("Total score: {}", total_score);
    println!("Average score: {}", average_score);
    println!("Total score with bonus: {}", total_with_bonus);

    //Part2 Assignment
    let mut final_score = total_score;

    final_score += bonus;
    println!("\nAfter bonus: {}", final_score);

    final_score -= 10;
    println!("After deduction: {}", final_score);

    //Part3 Comparison
    println!("\nTotal score is greater than 200: {}", total_score > 200);
    println!("Average score passed: {}", average_score >= passing_score);

    //Part4 Logical
    println!(
        "Student passed: {}",
        average_score >= passing_score
            && math_score >= passing_score
            && science_score >= passing_score
    );

    //Part5 Bitwise
    let a: u8 = 5;
    let b: u8 = 3;

    println!("\nA: {}", a);
    println!("B: {}", b);

    println!("Bitwise AND (A & B): {}", a & b);
    println!("Bitwise OR (A | B): {}", a | b);
    println!("Bitwise XOR (A ^ B): {}", a ^ b);
    println!("Left Shift (A << 1): {}", a << 1);
    println!("Right Shift (A >> 1): {}", a >> 1);
}
