use std::any::Any;

#[must_use]
pub struct Top(Box<dyn Any>);

impl Top {
    pub fn new<T: 'static>(x: T) -> Self {
        Self(Box::new(x))
    }
}

#[must_use]
pub struct With<A, B>(Box<dyn With_<A, B> + 'static>);
impl<A, B> With<A, B> {
    pub fn new<T: 'static>(
        seed: T,
        left: impl Fn(T) -> A + 'static,
        right: impl Fn(T) -> B + 'static,
    ) -> Self {
        Self(Box::new(WithImpl { seed, left, right }))
    }
    pub fn left(self) -> A {
        self.0.left()
    }
    pub fn right(self) -> B {
        self.0.right()
    }
}
trait With_<A, B> {
    fn left(self: Box<Self>) -> A;
    fn right(self: Box<Self>) -> B;
}
struct WithImpl<T, FA, FB> {
    seed: T,
    left: FA,
    right: FB,
}
impl<A, B, T, FA, FB> With_<A, B> for WithImpl<T, FA, FB>
where
    FA: Fn(T) -> A,
    FB: Fn(T) -> B,
{
    fn left(self: Box<Self>) -> A {
        let this = *self;
        (this.left)(this.seed)
    }
    fn right(self: Box<Self>) -> B {
        let this = *self;
        (this.right)(this.seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    fn with_comm<A, B>(x: With<A, B>) -> With<B, A>
    where
        A: 'static,
        B: 'static,
    {
        With::new(x, With::right, With::left)
    }

    #[allow(unused)]
    fn with_assoc1<A, B, C>(x: With<A, With<B, C>>) -> With<With<A, B>, C>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        With::new(
            x,
            |x| With::new(x, With::left, |x| x.right().left()),
            |x| x.right().right(),
        )
    }

    #[allow(unused)]
    fn with_assoc2<A, B, C>(x: With<With<A, B>, C>) -> With<A, With<B, C>>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        With::new(
            x,
            |x| x.left().left(),
            |x| With::new(x, |x| x.left().right(), With::right),
        )
    }

    #[allow(unused)]
    fn with_id1<A>(x: With<A, Top>) -> A
    where
        A: 'static,
    {
        x.left()
    }

    #[allow(unused)]
    fn with_id2<A>(x: A) -> With<A, Top>
    where
        A: 'static,
    {
        With::new(x, |x| x, Top::new)
    }

    #[allow(unused)]
    fn with_id3<A>(x: With<Top, A>) -> A
    where
        A: 'static,
    {
        x.right()
    }

    #[allow(unused)]
    fn with_id4<A>(x: A) -> With<Top, A>
    where
        A: 'static,
    {
        With::new(x, Top::new, |x| x)
    }

    #[allow(unused)]
    fn with_idem1<A>(x: With<A, A>) -> A
    where
        A: 'static,
    {
        x.left()
    }

    #[allow(unused)]
    fn with_idem2<A>(x: A) -> With<A, A>
    where
        A: 'static,
    {
        With::new(x, |x| x, |x| x)
    }
}
