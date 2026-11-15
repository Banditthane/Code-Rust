fn main() {
    let math_score: u32 = 80;
    let science_score: u32 = 70;
    let english_score: u32 = 90;
    let bonus: u32 = 10;
    let passing_score: u32 = 50;

    let total_score = math_score + science_score + english_score;
    let average_score = total_score / 3;
    let total_with_bonus = total_score + bonus;

    println!("Total score: {}", total_score);
    println!("Average score: {}", average_score);
    println!("Total score with bonus: {}", total_with_bonus);
}
