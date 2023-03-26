#[macro_use]
extern crate slog;

mod enemy;
mod player;
mod weapon;

// crate 表示当前项目根目录
use crate::enemy::Enemy;
use crate::player::Player;

use std::{thread, time::Duration};

use rand::Rng;
use slog::{Drain, Logger};
use slog_async::Async;

fn main() {
    let drain = slog_json::Json::new(std::io::stdout())
        .add_default_keys()
        .build()
        .fuse();
    let async_drain = Async::new(drain).build().fuse();
    // 本程序的版本号 Cargo.toml  package -> version
    let game_info = format!("v{}", env!("CARGO_PKG_VERSION"));
    let root_log_context = o!("Super Cool Game"=>game_info);
    let root_logger = Logger::root(async_drain, root_log_context);
    let game = Game {
        logger: root_logger.clone(),
        enemy: Enemy::new(&root_logger, "Malice"),
        player: Player::new(&root_logger, "Bob"),
    };
    game.simulate();
}

struct Game {
    logger: Logger,
    enemy: Enemy,
    player: Player,
}

impl Game {
    fn simulate(&self) {
        info!(self.logger, "Launching game!");
        let enemy_or_player: Vec<&dyn PlayingCharacter> = vec![&self.enemy, &self.player];
        loop {
            let mut rng = rand::thread_rng();
            let a = rng.gen_range(500..1000);
            thread::sleep(Duration::from_millis(a));
            let player = enemy_or_player[if a % 2 == 0 { 1 } else { 0 }];
            player.shoot();
        }
    }
}

pub trait PlayingCharacter {
    fn shoot(&self);
}
