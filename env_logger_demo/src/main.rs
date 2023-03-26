use log::debug;
use user_auth::User;

fn main() {
    println!("Hello, world!");
    env_logger::init();
    debug!("env logger demo started");
    let user = User::new("bobo", "super_secret");
    user.sign_in("super_secret");
    user.sign_in("super_sekret");
}
