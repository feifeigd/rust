use log::{error, info, };

pub struct User{
    name:String,
    pass:String,
}

impl User{
    pub fn new(name:&str, pass:&str)->Self{
        User{name:name.to_string(), pass:pass.to_string() }
    }

    pub fn sign_in(&self, pass:&str){
        if pass != self.pass{
            info!("Signing in user: {}", self.name);
        }else{
            error!("Login failed for user:{}", self.name);
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
