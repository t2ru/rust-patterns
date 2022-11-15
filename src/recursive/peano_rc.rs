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

    pub fn dec(&self) -> Option<&Self> {
        match self.inner() {
            PeanoInner::O => None,
            PeanoInner::S(x) => Some(x),
        }
    }

    pub fn count(&self) -> i32 {
        match self.dec() {
            None => 0,
            Some(x) => x.count() + 1,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match other.dec() {
            None => self.clone(),
            Some(x) => self.cons().add(x),
        }
    }

    pub fn sub(&self, other: &Self) -> Option<&Self> {
        match (self.dec(), other.dec()) {
            (_, None) => Some(self),
            (None, _) => None,
            (Some(x), Some(y)) => x.sub(y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Peano;

    #[test]
    fn test_add() {
        let zero = Peano::zero();
        let three = Peano::zero().cons().cons().cons();
        let five = Peano::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.add(&zero).count(), 0);
        assert_eq!(zero.add(&three).count(), 3);
        assert_eq!(three.add(&zero).count(), 3);
        assert_eq!(three.add(&five).count(), 8);
        assert_eq!(five.add(&three).count(), 8);
    }

    #[test]
    fn test_sub() {
        let zero = Peano::zero();
        let three = Peano::zero().cons().cons().cons();
        let five = Peano::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.sub(&zero).map(|x| x.count()), Some(0));
        assert_eq!(three.sub(&three).map(|x| x.count()), Some(0));
        assert_eq!(five.sub(&five).map(|x| x.count()), Some(0));
        assert_eq!(zero.sub(&three).map(|x| x.count()), None);
        assert_eq!(three.sub(&zero).map(|x| x.count()), Some(3));
        assert_eq!(five.sub(&three).map(|x| x.count()), Some(2));
        assert_eq!(three.sub(&five).map(|x| x.count()), None);
    }
}
