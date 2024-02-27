use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Clone, Debug)]
pub struct Technologies {
    pub established_technology: HashMap<Technology, i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Technology {
    Primitive,
    Developed
}

pub static MAX_TECH_LEVEL: Lazy<HashMap<Technology, i32>> = Lazy::new(|| {
    let mut max_tech_level = HashMap::<Technology, i32>::new();
    max_tech_level.insert(Technology::Primitive, 10);
    max_tech_level.insert(Technology::Developed, 100);
    max_tech_level
});