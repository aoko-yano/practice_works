use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Cultures {
    pub established_culture: HashMap<Culture, i32>,
}

#[derive(Clone, Debug)]
pub enum Culture {}