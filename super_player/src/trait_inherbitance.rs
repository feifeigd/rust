pub trait Vehicle {
    fn get_price(&self) -> u64;
}

pub trait Car: Vehicle {
    fn model(&self) -> String;
}

pub struct TeslaRoadster {
    model: String,
    release_date: u16,
}

impl TeslaRoadster {
    pub fn new(model: &str, release_date: u16) -> Self {
        Self {
            model: model.to_string(),
            release_date,
        }
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200_000
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        self.model.to_string()
    }
}
