/* 
 * Base deffinition of a resorce
 */
pub trait Resource {
    /// Return the current value of the resource.
    fn current(&self) -> i32;

    /// Return the maximum possible value of the resource.
    fn max(&self) -> i32;
}

/* 
 * List of resources traites
 */
/// A damageable resource
pub trait Damage: Resource {
    // To impl CanDamage, you have to impl Resource first
    fn damage(self, amount: i32);
}
/// A spendable resource
pub trait Spend: Resoruce {
    fn spend(self, amount: i32);
}
// A per turn generating resource?
// A per condition generating resource?
// a more general generation trait? that do above on impl?
// can we search by traits easily?

/* ========================================================================== *\
 *
 *      List of resorces
 * 
 * ========================================================================== */
 //should all defined traits be in there own files?
/* 
 * Health
 */
pub struct Health {
    current: i32,
    max: i32,
}

impl Health {}

impl Resource for Health {
    fn current(&self) -> i32 {
        self.current
    }
    fn max(&self) -> i32 {
        self.max
    }
}

impl Damage for Health {
    fn damage(mut self, amount: i32) {
        self.current -= amount;
    }
}

/* 
 * Mana?
 */

/*
 * Rage?
 */

/*
 * different resources based on what?
 */
