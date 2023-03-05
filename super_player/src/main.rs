mod media;
mod trait_bound_basic;
mod trait_bound_functions;
mod trait_bound_intro;
mod trait_inherbitance;

use media::*;
use trait_bound_basic::*;
use trait_bound_functions::*;
//use trait_bound_functions::{eat,Food};
use trait_bound_intro::*;
use trait_inherbitance::*;

fn main() {
    println!("Super player!");

    let audio = Audio("ambient_music.mp3".to_string());
    let video = Audio("big_buck_bunny.mkv".to_string());
    audio.play();
    video.play();

    let roadster = TeslaRoadster::new("Tesla Roadster II", 2020);
    println!(
        "{} is priced at ${} ",
        roadster.model(),
        roadster.get_price()
    );

    let game = Game;
    game.load(Enemy);
    game.load(Hero);

    add_thing(2, 2);

    let apple = Food(Apple);
    eat(apple);
}

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0)
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0)
    }
}
