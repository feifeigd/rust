

use std::fmt::{Display, Formatter, Result};

use slog::Logger;

#[derive(Debug)]
pub struct PlasmaCannon(pub Logger);

impl PlasmaCannon {
    pub fn fire(&self) {
        info!(self.0, "Pew Pew !!");
    }
}

impl Display for PlasmaCannon {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, stringify!(Self))
    }
}

#[derive(Debug)]
pub struct RailGun(pub Logger);

impl RailGun {
    pub fn fire(&self) {
        info!(self.0, "Swooh !!");
    }
}

impl Display for RailGun {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, stringify!(Self))
    }
}
