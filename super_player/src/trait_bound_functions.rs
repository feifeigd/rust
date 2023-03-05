use std::fmt::Debug;

pub trait Eatable {
    fn eat(&self);
}

#[derive(Debug)]
pub struct Food<T>(pub T);

#[derive(Debug)]
pub struct Apple;

impl<T: Debug> Eatable for Food<T>
{
    fn eat(&self) {
        println!("Eating {:?}", self)
    }
}

pub fn eat<T:Eatable>(val: T) {
    val.eat();
}
