pub mod technology;
pub mod culture;

use culture::Cultures;
use technology::Technologies;

#[derive(Clone, Debug)]
pub struct Society {
    pub population: Population,
    pub cultures: Cultures,
    pub technologies: Technologies,
}

#[derive(Clone, Debug)]
pub struct Population {
    pub number: usize,
}