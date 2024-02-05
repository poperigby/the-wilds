/*
 * Combat loop
 */
//includes
mod character;
mod effects;

//should this be public?
pub struct CombatCharacter{
    character: CharacterSheet,
    roundEff:  Vec<Box<dyn effect>>,
    turnEff:   Vec<Box<dyn effect>>
}

pub fn combatloop(){
    
}
