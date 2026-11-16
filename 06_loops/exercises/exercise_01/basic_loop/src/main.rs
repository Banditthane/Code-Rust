fn main() {
    let mut count = 1;

    loop {
        println!("Count: {}", count);

        if count == 10 {
            break;
        }
        count += 1;
    }
}
