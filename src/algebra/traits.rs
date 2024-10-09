use num_bigint::BigUint;

/// Base trait that defines the associated type `Element`.
pub trait Algebra {
    type Element;
}

/// Trait representing a mathematical ring.
pub trait Ring: Algebra {
    /// Adds two elements within the ring.
    fn add(&self, a: &Self::Element, b: &Self::Element) -> Self::Element;

    /// Multiplies two elements within the ring.
    fn mul(&self, a: &Self::Element, b: &Self::Element) -> Self::Element;

    /// Returns the additive identity of the ring.
    fn zero(&self) -> Self::Element;

    /// Returns the multiplicative identity of the ring.
    fn one(&self) -> Self::Element;
}

/// Trait representing a mathematical group.
pub trait Group: Algebra {
    /// Combines two elements within the group.
    fn combine(&self, a: &Self::Element, b: &Self::Element) -> Self::Element;

    /// Returns the identity element of the group.
    fn identity(&self) -> Self::Element;

    /// Returns the inverse of an element within the group.
    fn inverse(&self, a: &Self::Element) -> Option<Self::Element>;

    /// Raises an element to a non-negative integer power.
    fn pow(&self, a: &Self::Element, exp: &BigUint) -> Self::Element;
}

/// Trait representing a mathematical field.
/// Combines both Ring and Group traits.
pub trait Field: Ring + Group {}
