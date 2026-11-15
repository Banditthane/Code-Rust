fn main() {
    let age = 20;
    let has_ticket = true;
    let is_member = false;
    let is_banned = false;

    //Condition 1
    println!("Can join: {}", age >= 18 && has_ticket && !is_banned);

    //Condition 2
    println!(
        "Has privilege: {}",
        is_member || (age >= 18 && has_ticket && !is_banned)
    );

    //Condition 3
    println!(
        "Final permission: {}",
        (age >= 18 && has_ticket && !is_banned) || is_member
    );
}
