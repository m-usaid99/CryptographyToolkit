use crate::algebra::traits::Ring;

pub struct GenericVector<R: Ring> {
    elements: Vec<R::Element>,
    ring: R,
}

impl<R: Ring> GenericVector<R> {
    pub fn new(ring: R) -> Self {
        GenericVector {
            elements: Vec::new(),
            ring,
        }
    }

    pub fn push(&mut self, element: R::Element) {
        self.elements.push(element);
    }
}
