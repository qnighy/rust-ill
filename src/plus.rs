pub enum Zero {}

#[must_use]
pub enum Plus<A, B> {
    Left(A),
    Right(B),
}

pub use Plus::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    fn plus_comm<A, B>(x: Plus<A, B>) -> Plus<B, A>
    where
        A: 'static,
        B: 'static,
    {
        match x {
            Left(x) => Right(x),
            Right(x) => Left(x),
        }
    }

    #[allow(unused)]
    fn plus_assoc1<A, B, C>(x: Plus<A, Plus<B, C>>) -> Plus<Plus<A, B>, C>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        match x {
            Left(x) => Left(Left(x)),
            Right(Left(x)) => Left(Right(x)),
            Right(Right(x)) => Right(x),
        }
    }

    #[allow(unused)]
    fn plus_assoc2<A, B, C>(x: Plus<Plus<A, B>, C>) -> Plus<A, Plus<B, C>>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        match x {
            Left(Left(x)) => Left(x),
            Left(Right(x)) => Right(Left(x)),
            Right(x) => Right(Right(x)),
        }
    }

    #[allow(unused)]
    fn plus_id1<A>(x: Plus<A, Zero>) -> A
    where
        A: 'static,
    {
        match x {
            Left(x) => x,
            Right(z) => match z {},
        }
    }

    #[allow(unused)]
    fn plus_id2<A>(x: A) -> Plus<A, Zero>
    where
        A: 'static,
    {
        Left(x)
    }

    #[allow(unused)]
    fn plus_id3<A>(x: Plus<Zero, A>) -> A
    where
        A: 'static,
    {
        match x {
            Left(z) => match z {},
            Right(x) => x,
        }
    }

    #[allow(unused)]
    fn plus_id4<A>(x: A) -> Plus<Zero, A>
    where
        A: 'static,
    {
        Right(x)
    }

    #[allow(unused)]
    fn plus_idem1<A>(x: Plus<A, A>) -> A
    where
        A: 'static,
    {
        match x {
            Left(x) => x,
            Right(x) => x,
        }
    }

    #[allow(unused)]
    fn plus_idem2<A>(x: A) -> Plus<A, A>
    where
        A: 'static,
    {
        Left(x)
    }
}
