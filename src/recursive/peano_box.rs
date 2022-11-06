#[derive(Clone)]
enum PeanoKind {
    O,
    S(Peano),
}

#[derive(Clone)]
struct Peano(Box<PeanoKind>);

impl Peano {
    fn zero() -> Self {
        Self(Box::new(PeanoKind::O))
    }

    fn cons(&self) -> Self {
        Self(Box::new(PeanoKind::S(self.clone())))
    }

    fn inner(&self) -> &PeanoKind {
        &*self.0
    }

    fn count(&self) -> i32 {
        match self.inner() {
            PeanoKind::O => 0,
            PeanoKind::S(x) => x.count() + 1,
        }
    }

    fn add(&self, other: &Self) -> Self {
        match self.inner() {
            PeanoKind::O => other.clone(),
            PeanoKind::S(x) => x.add(&other.cons())
        }
    }
}

#[test]
fn test() {
    let three = Peano::zero().cons().cons().cons();
    let five = Peano::zero().cons().cons().cons().cons().cons();
    assert_eq!(three.count(), 3);
    assert_eq!(five.count(), 5);
    assert_eq!(three.add(&five).count(), 8);
}
