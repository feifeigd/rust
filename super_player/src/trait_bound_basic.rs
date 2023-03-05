use std::ops::Add;

pub fn add_thing<T: Add>(fst: T, snd: T) {
    let _ = fst + snd;
}
