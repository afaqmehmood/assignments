fn main() {
    let number = 13;
    println!("Tell me about {}", number);
    println!("Tell me about {}", number);
    match number {
        // Match a single value
        13 => println!("Integer"),
        _ => println!("Ain't special"),
    }
}
