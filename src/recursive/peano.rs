#[derive(Clone)]
pub enum PeanoMatcher<Peano> {
    O,
    S(Peano),
}

use PeanoMatcher::{O, S};

trait Peano: Clone {
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

#[cfg(test)]
mod rc_tests {
    use super::{Peano, PeanoMatcher};
    use std::rc::Rc;

    #[derive(Clone)]
    pub struct RcPeano(Rc<PeanoMatcher<Self>>);

    impl Peano for RcPeano {
        fn new(inner: PeanoMatcher<Self>) -> Self {
            Self(Rc::new(inner))
        }

        fn matcher(&self) -> &PeanoMatcher<Self> {
            &self.0
        }
    }

    #[test]
    fn test_rc_add() {
        let zero = RcPeano::zero();
        let three = RcPeano::zero().cons().cons().cons();
        let five = RcPeano::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.add(&zero).count(), 0);
        assert_eq!(zero.add(&three).count(), 3);
        assert_eq!(three.add(&zero).count(), 3);
        assert_eq!(three.add(&five).count(), 8);
        assert_eq!(five.add(&three).count(), 8);
    }

    #[test]
    fn test_rc_sub() {
        let zero = RcPeano::zero();
        let three = RcPeano::zero().cons().cons().cons();
        let five = RcPeano::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.sub(&zero).map(|x| x.count()), Some(0));
        assert_eq!(three.sub(&three).map(|x| x.count()), Some(0));
        assert_eq!(five.sub(&five).map(|x| x.count()), Some(0));
        assert_eq!(zero.sub(&three).map(|x| x.count()), None);
        assert_eq!(three.sub(&zero).map(|x| x.count()), Some(3));
        assert_eq!(five.sub(&three).map(|x| x.count()), Some(2));
        assert_eq!(three.sub(&five).map(|x| x.count()), None);
    }
}

#[cfg(test)]
mod box_tests {
    use super::{Peano, PeanoMatcher};

    #[derive(Clone)]
    pub struct BoxPeano(Box<PeanoMatcher<Self>>);

    impl Peano for BoxPeano {
        fn new(inner: PeanoMatcher<Self>) -> Self {
            Self(Box::new(inner))
        }

        fn matcher(&self) -> &PeanoMatcher<Self> {
            &self.0
        }
    }

    #[test]
    fn test_box_add() {
        let zero = BoxPeano::zero();
        let three = BoxPeano::zero().cons().cons().cons();
        let five = BoxPeano::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.add(&zero).count(), 0);
        assert_eq!(zero.add(&three).count(), 3);
        assert_eq!(three.add(&zero).count(), 3);
        assert_eq!(three.add(&five).count(), 8);
        assert_eq!(five.add(&three).count(), 8);
    }

    #[test]
    fn test_box_sub() {
        let zero = BoxPeano::zero();
        let three = BoxPeano::zero().cons().cons().cons();
        let five = BoxPeano::zero().cons().cons().cons().cons().cons();
        assert_eq!(zero.sub(&zero).map(|x| x.count()), Some(0));
        assert_eq!(three.sub(&three).map(|x| x.count()), Some(0));
        assert_eq!(five.sub(&five).map(|x| x.count()), Some(0));
        assert_eq!(zero.sub(&three).map(|x| x.count()), None);
        assert_eq!(three.sub(&zero).map(|x| x.count()), Some(3));
        assert_eq!(five.sub(&three).map(|x| x.count()), Some(2));
        assert_eq!(three.sub(&five).map(|x| x.count()), None);
    }
}
