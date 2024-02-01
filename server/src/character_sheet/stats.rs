/* 
 * Deffition of stats
 */

// stats are static and will be garenteed to exist on all things.
//  
// All stats are useful in both offense and defence as abilities are based on 
// one stat and target a different one. So stats need a defined side of both. 
#[derive(Debug)]
pub struct StatBlock{
    //           Full name    Attack def,    Defence def,   
    pub str: i32, // Strength,    Hit hard,      take a hit,       Tank,
    pub dex: i32, // Dexterity,   Hit fast,      don't get hit,    dodge
    pub per: i32, // perception,  Hit percisely, noticing things,   Might need a better name.
    pub wis: i32, // wisdom,      Natural Power, resist dots?
    pub int: i32, // Intelegence, Arcane Power,  mind over matter, Alternativly Cunning
    pub cha: i32, // Charisma,    Occult Power,  Charm effects?

    // I want something gauged twards persision. crits
    //
    // more medium?
    //end: i32 // endurance,   Hit even?  
}
// I don't know how I feel about int being intelegence.
// 
// Things have overlap, and thats intentional, players shouldn't want to only 
// add to one stat, they should want a veriety to define there playstyle. 