use axum::extract::Query;
use std::collections::HashMap;

pub type QueryParam = Query<HashMap<String, String>>;

pub type BandRow = (i32, String, String, String);
