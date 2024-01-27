/* 
 * Deffition of stats
 */

// stats are static and will be garenteed to exist on all things.
//  
// All stats are useful in both offense and defence as abilities are based on 
// one stat and target a different one. So stats need a defined side of both. 

pub struct Statblock{
    //           Full name    Attack def,    Defence def,   
    str: i32, // Strength,    Hit hard,      take a hit,       Tank,
    dex: i32, // Dexterity,   Hit fast,      don't get hit,    dodge
    wis: i32, // wisdom,      Natural Power, resist dots?
    int: i32, // Intelegence, Arcane Power,  mind over matter, Alternativly Cunning
    cha: i32, // Charisma,    Occult Power,  

    // I want something gauged twards persistion. crits
    //per: i32, // persception, Hit percisely, block things?   Might need a better name axing this for now.
    //
    // more medium?
    //end: i32 // endurance,   Hit even?  
}
// I don't know how I feel about int being intelegence.
// 
// Things have overlap, and thats intentional, players shouldn't want to only 
// add to one stat, they should want a veriety to define there playstyle. 