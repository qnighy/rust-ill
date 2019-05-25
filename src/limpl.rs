#[must_use]
pub struct Limpl<A, B>(Box<dyn FnOnce(A) -> B>);

impl<A, B> Limpl<A, B> {
    pub fn new(f: impl FnOnce(A) -> B + 'static) -> Self {
        Self(Box::new(f))
    }

    pub fn apply(self, x: A) -> B {
        self.0(x)
    }
}
