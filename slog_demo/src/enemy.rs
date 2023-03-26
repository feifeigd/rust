use slog::Logger;

use crate::PlayingCharacter;
use crate::weapon::RailGun;

pub struct Enemy {
    name: String,
    logger: Logger,
    weapon: RailGun,
}

impl Enemy {
    pub fn new(logger: &Logger, name: &str) -> Self {
        let enemy_log = logger.new(o!("Player"=>format!("{}", name)));
        let weapon_log = enemy_log.new(o!("RailGun"=>"S12"));
        Self {
            logger: enemy_log,
            name: name.to_string(),
            weapon: RailGun(weapon_log),
        }
    }
}
impl PlayingCharacter for Enemy {
    fn shoot(&self) {
        warn!(self.logger, "{} shooting with ", self.name);
        self.weapon.fire();
    }
}
