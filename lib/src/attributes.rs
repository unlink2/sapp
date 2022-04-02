use std::collections::HashMap;

pub enum Attributes {
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<Attributes>),
    Struct(HashMap<Attributes, Attributes>)
}
