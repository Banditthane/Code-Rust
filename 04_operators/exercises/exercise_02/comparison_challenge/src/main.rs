fn main() {
    let age = 20;
    let score = 75;
    let mut passing_score = 50;

    println!("Age is {}: {}", age, age == 20);
    println!("Age is not 18: {}", age == 20);
    println!(
        "Score is greater than {}: {}",
        passing_score,
        score > passing_score
    );

    passing_score += 50;
    println!(
        "Score is less than {}: {}",
        passing_score,
        score < passing_score
    );
    passing_score = 50;
    println!("Passed: {}", score >= passing_score);
    println!(
        "Score is less than or equal to {}: {}",
        score,
        score >= passing_score
    );
}
