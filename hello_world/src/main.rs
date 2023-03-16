mod first_macro;

fn main() {
    println!("Hello, world!");
    println!("{0}, this is {1}. {1} this is {0}.", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("{} of {:b} people know binary, the other half dont't", 1, 2);
    // 打印宽度控制
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);

    let mut input = String::new();
    scanline!(input);
    println!("{:?}", input);
}
