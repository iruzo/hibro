use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use crate::data;

pub static CONNECTIONS: Lazy<Arc<Mutex<Vec<data::model::connection::Connection>>>> = Lazy::new(|| {
    let cn = Arc::new(Mutex::new(Vec::new()));
    cn
});
