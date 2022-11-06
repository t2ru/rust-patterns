use std::rc::Rc;

enum PeanoInner {
    O,
    S(Peano),
}

#[derive(Clone)]
pub struct Peano(Rc<PeanoInner>);

impl Peano {
    fn new(inner: PeanoInner) -> Self {
        Self(Rc::new(inner))
    }

    fn inner(&self) -> &PeanoInner {
        &*self.0
    }

    pub fn zero() -> Self {
        Self::new(PeanoInner::O)
    }

    pub fn cons(&self) -> Self {
        Self::new(PeanoInner::S(self.clone()))
    }

    pub fn dec(&self) -> Option<Self> {
        match self.inner() {
            PeanoInner::O => None,
            PeanoInner::S(x) => Some(x.clone()),
        }
    }

    pub fn count(&self) -> i32 {
        match self.dec() {
            None => 0,
            Some(x) => x.count() + 1,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match self.dec() {
            None => other.clone(),
            Some(x) => x.add(&other.cons()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Peano;

    #[test]
    fn test() {
        let three = Peano::zero().cons().cons().cons();
        let five = Peano::zero().cons().cons().cons().cons().cons();
        assert_eq!(three.count(), 3);
        assert_eq!(five.count(), 5);
        assert_eq!(three.add(&five).count(), 8);
    }
}
