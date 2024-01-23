pub trait Resource {
    /// Return the current value of the resource.
    fn current(&self) -> i32;

    /// Return the maximum possible value of the resource.
    fn max(&self) -> i32;
}

/// A damageable resource
pub trait Damage: Resource {
    // To impl CanDamage, you have to impl Resource first
    fn damage(self, amount: i32);
}
