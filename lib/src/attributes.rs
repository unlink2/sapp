use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Attributes {
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<Attributes>),
    Struct(HashMap<String, Attributes>)
}
