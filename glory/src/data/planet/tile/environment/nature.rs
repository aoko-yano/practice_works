
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AreaType {
    Green,
    Sea,
    Desert,
    Snow,
    Forest
}

#[derive(Clone, Debug)]
pub struct Nature {
    pub area_type: AreaType,
}