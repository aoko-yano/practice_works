pub mod biological_resource;
pub mod natural_resource;
pub mod nature;

use biological_resource::BiologicalResource;
use natural_resource::NaturalResources;
use nature::Nature;

#[derive(Clone, Debug)]
pub struct Environment {
    pub biological_resource: BiologicalResource,
    pub natural_resources: NaturalResources,
    pub nature: Nature,
}