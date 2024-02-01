/*
 * Character Sheet (base character info)
 */
//modules
mod stats;
mod resource;
//actions
//inventory

use stats::StatBlock;
use resource::{Damage, Resource, Health}; //Does this grab exclusivly those? do they need to be spessified?

#[derive(Debug)]
pub struct CharacterSheet {
    stats: StatBlock,
    resources: Vec<Box<dyn Resource>>,
    //potential actions
    //Items? (effects potential actions)

}

//this needs insertion functions so that things get added correctly. 

//tests
#[cfg(test)]
mod tests {
    use super::*;
    //use resource::*;

    #[test]
    fn sheet() {
        let sheet = CharacterSheet {
            stats: StatBlock {
                str: 5,
                dex: 5,
                per: 5,
                wis: 5,
                int: 5,
                cha: 5,
            },
            resources: vec![],
        };
        dbg!(sheet); 
    }

    #[test]
    fn health() {
        let mut sheet = CharacterSheet {
            stats: StatBlock {
                str: 7,
                dex: 7,
                per: 7,
                wis: 7,
                int: 7,
                cha: 7,
            },
            resources: vec![],
        };
        sheet.resources.push(Box::new(Health::new(25, None)));
        dbg!(sheet); 
    }
}
