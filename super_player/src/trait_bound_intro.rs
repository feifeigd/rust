pub struct Game;
pub struct Enemy;
pub struct Hero;

trait Loadable{
    fn init(&self);
}

impl Game {
    pub fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

impl Loadable for Enemy{
    fn init(&self){
        println!("Enemy loaded")
    }
}

impl Loadable for Hero{
    fn init(&self){
        println!("Hero loaded")
    }
}
