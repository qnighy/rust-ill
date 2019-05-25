use std::sync::Arc;

pub struct Bang<T>(Arc<Fn() -> T + 'static>);

impl<T> Bang<T> {
    pub fn new(f: impl Fn() -> T + 'static) -> Self {
        Self(Arc::new(f))
    }

    pub fn elim(&self) -> T {
        self.0()
    }
}

impl<T> Clone for Bang<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    fn bang_comonad1<A>(x: Bang<A>) -> A
    where
        A: 'static,
    {
        x.elim()
    }

    #[allow(unused)]
    fn bang_comonad2<A>(x: Bang<A>) -> Bang<Bang<A>>
    where
        A: 'static,
    {
        Bang::new(move || x.clone())
    }
}
