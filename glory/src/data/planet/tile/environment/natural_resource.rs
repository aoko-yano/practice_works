use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct NaturalResources {
    pub existing_natural_resource: HashMap<NaturalResource, i32>,
}

#[derive(Clone, Debug)]
pub enum NaturalResource {}