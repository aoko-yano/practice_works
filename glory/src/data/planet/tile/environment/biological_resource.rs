use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BiologicalResource {
    pub living_species: HashMap<Species, i32>
}

#[derive(Clone, Debug)]
pub enum Species {}