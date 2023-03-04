mod media;
use media::*;

fn main() {
    println!("Super player!");

    let audio = Audio("ambient_music.mp3".to_string());
    let video = Audio("big_buck_bunny.mkv".to_string());
    audio.play();
    video.play();
}

struct Audio(String);
struct Video(String);

impl Playable for Audio{
    fn play(&self){
        println!("Now playing: {}", self.0)
    }
}

impl Playable for Video{
    fn play(&self){
        println!("Now playing: {}", self.0)
    }
}
