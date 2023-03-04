fn main() {
    println!("Hello, world!");
    let score = 2048;
    increase_by(score, 30);
    println!("You made {} points", score);
}

fn increase_by(mut val: u32, how_much: u32) {
    val += how_much;
}
