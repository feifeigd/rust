use crate::links::Links;

use actix::{Actor, Addr, SyncArbiter, SyncContext};

use std::sync::{Arc, Mutex};

const DB_THREADS: usize = 3;

pub struct Db {
    pub inner: Arc<Mutex<Links>>,
}

impl Db {
    pub fn new(s: Arc<Mutex<Links>>) -> Self {
        Db { inner: s }
    }
}
impl Actor for Db {
    type Context = SyncContext<Self>;
}

#[derive(Clone)]
pub struct State {
    pub inner: Addr<Db>,
}

impl State {
    pub fn init() -> Self {
        let state = Arc::new(Mutex::new(Links::new()));
        let state = SyncArbiter::start(DB_THREADS, move || Db::new(state.clone()));
        State { inner: state }
    }
}
