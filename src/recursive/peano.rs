#[derive(Clone)]
pub enum PeanoMatcher<Peano> {
    O,
    S(Peano),
}

use PeanoMatcher::{O, S};

trait Peano where Self: Clone {
    fn new(inner: PeanoMatcher<Self>) -> Self;
    fn matcher(&self) -> &PeanoMatcher<Self>;

    fn zero() -> Self {
        Self::new(PeanoMatcher::O)
    }

    fn cons(&self) -> Self {
        Self::new(PeanoMatcher::S(self.clone()))
    }

    fn count(&self) -> i32 {
        match self.matcher() {
            O => 0,
            S(x) => x.count() + 1,
        }
    }

    fn add(&self, other: &Self) -> Self {
        match other.matcher() {
            O => self.clone(),
            S(x) => self.cons().add(x),
        }
    }

    fn sub(&self, other: &Self) -> Option<&Self> {
        match (self.matcher(), other.matcher()) {
            (_, O) => Some(self),
            (O, _) => None,
            (S(x), S(y)) => x.sub(y),
        }
    }
}


use std::rc::Rc;

#[derive(Clone)]
pub struct PeanoRc(Rc<PeanoMatcher<Self>>);

impl Peano for PeanoRc {
    fn new(inner: PeanoMatcher<Self>) -> Self {
        Self(Rc::new(inner))
    }

    fn matcher(&self) -> &PeanoMatcher<Self> {
        &*self.0
    }
}


#[derive(Clone)]
pub struct PeanoBox(Box<PeanoMatcher<Self>>);

impl Peano for PeanoBox {
    fn new(inner: PeanoMatcher<Self>) -> Self {
        Self(Box::new(inner))
    }

    fn matcher(&self) -> &PeanoMatcher<Self> {
        &*self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Peano, PeanoRc, PeanoBox};

    #[test]
    fn test_rc_add() {
        let zero = PeanoRc::zero();
        let three = PeanoRc::zero().cons().cons().cons();
        let five = PeanoRc::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.add(&zero).count(), 0);
        assert_eq!(zero.add(&three).count(), 3);
        assert_eq!(three.add(&zero).count(), 3);
        assert_eq!(three.add(&five).count(), 8);
        assert_eq!(five.add(&three).count(), 8);
    }

    #[test]
    fn test_rc_sub() {
        let zero = PeanoRc::zero();
        let three = PeanoRc::zero().cons().cons().cons();
        let five = PeanoRc::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.sub(&zero).map(|x| x.count()), Some(0));
        assert_eq!(three.sub(&three).map(|x| x.count()), Some(0));
        assert_eq!(five.sub(&five).map(|x| x.count()), Some(0));
        assert_eq!(zero.sub(&three).map(|x| x.count()), None);
        assert_eq!(three.sub(&zero).map(|x| x.count()), Some(3));
        assert_eq!(five.sub(&three).map(|x| x.count()), Some(2));
        assert_eq!(three.sub(&five).map(|x| x.count()), None);
    }

    #[test]
    fn test_box_add() {
        let zero = PeanoBox::zero();
        let three = PeanoBox::zero().cons().cons().cons();
        let five = PeanoBox::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.add(&zero).count(), 0);
        assert_eq!(zero.add(&three).count(), 3);
        assert_eq!(three.add(&zero).count(), 3);
        assert_eq!(three.add(&five).count(), 8);
        assert_eq!(five.add(&three).count(), 8);
    }

    #[test]
    fn test_box_sub() {
        let zero = PeanoBox::zero();
        let three = PeanoBox::zero().cons().cons().cons();
        let five = PeanoBox::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.sub(&zero).map(|x| x.count()), Some(0));
        assert_eq!(three.sub(&three).map(|x| x.count()), Some(0));
        assert_eq!(five.sub(&five).map(|x| x.count()), Some(0));
        assert_eq!(zero.sub(&three).map(|x| x.count()), None);
        assert_eq!(three.sub(&zero).map(|x| x.count()), Some(3));
        assert_eq!(five.sub(&three).map(|x| x.count()), Some(2));
        assert_eq!(three.sub(&five).map(|x| x.count()), None);
    }
}
