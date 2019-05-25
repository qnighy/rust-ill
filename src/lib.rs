mod bang;
mod plus;
mod tensor;
mod with;

pub use bang::Bang;
pub use plus::{Left, Plus, Right, Zero};
pub use tensor::{One, Tensor};
pub use with::{Top, With};

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    fn distrib1<A, B, C>(x: Tensor<Plus<A, B>, C>) -> Plus<Tensor<A, C>, Tensor<B, C>>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        match x {
            Tensor(Left(x), y) => Left(Tensor(x, y)),
            Tensor(Right(x), y) => Right(Tensor(x, y)),
        }
    }

    #[allow(unused)]
    fn distrib2<A, B, C>(x: Plus<Tensor<A, C>, Tensor<B, C>>) -> Tensor<Plus<A, B>, C>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        match x {
            Left(Tensor(x, y)) => Tensor(Left(x), y),
            Right(Tensor(x, y)) => Tensor(Right(x), y),
        }
    }

    #[allow(unused)]
    fn distrib3<A>(x: Tensor<Zero, A>) -> Zero
    where
        A: 'static,
    {
        match x.0 {}
    }

    #[allow(unused)]
    fn distrib4<A>(x: Zero) -> Tensor<Zero, A>
    where
        A: 'static,
    {
        match x {}
    }

    #[allow(unused)]
    fn modular1<A, B, C>(x: Tensor<With<A, B>, C>) -> With<Tensor<A, C>, Tensor<B, C>>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        With::new(x, |x| Tensor(x.0.left(), x.1), |x| Tensor(x.0.right(), x.1))
    }

    #[allow(unused)]
    fn modular2<A>(x: Tensor<Top, A>) -> Top
    where
        A: 'static,
    {
        Top::new(x)
    }

    #[allow(unused)]
    fn exponent1<A, B>(x: Bang<With<A, B>>) -> Tensor<Bang<A>, Bang<B>>
    where
        A: 'static,
        B: 'static,
    {
        let x2 = x.clone();
        Tensor(
            Bang::new(move || x.elim().left()),
            Bang::new(move || x2.elim().right()),
        )
    }

    #[allow(unused)]
    fn exponent2<A, B>(x: Tensor<Bang<A>, Bang<B>>) -> Bang<With<A, B>>
    where
        A: 'static,
        B: 'static,
    {
        Bang::new(move || With::new((x.0.clone(), x.1.clone()), |x| x.0.elim(), |x| x.1.elim()))
    }

    #[allow(unused)]
    fn exponent3(x: Bang<Top>) -> One {
        One
    }

    #[allow(unused)]
    fn exponent4(x: One) -> Bang<Top> {
        Bang::new(move || Top::new(()))
    }
}
