mod stats;
mod resource;
mod effect;

use resource::{Damage, Resource}; //Does this grab exclusivly those? do they need to be spessified?

pub struct CharacterSheet {
    stats: Statblock;
    resources: Vec<Box<dyn Resource>>,
    

}
