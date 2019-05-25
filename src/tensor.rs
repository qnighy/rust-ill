pub struct One;

#[must_use]
pub struct Tensor<A, B>(pub A, pub B);

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    fn tensor_comm<A, B>(x: Tensor<A, B>) -> Tensor<B, A>
    where
        A: 'static,
        B: 'static,
    {
        Tensor(x.1, x.0)
    }

    #[allow(unused)]
    fn tensor_assoc1<A, B, C>(x: Tensor<A, Tensor<B, C>>) -> Tensor<Tensor<A, B>, C>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        Tensor(Tensor(x.0, (x.1).0), (x.1).1)
    }

    #[allow(unused)]
    fn tensor_assoc2<A, B, C>(x: Tensor<Tensor<A, B>, C>) -> Tensor<A, Tensor<B, C>>
    where
        A: 'static,
        B: 'static,
        C: 'static,
    {
        Tensor((x.0).0, Tensor((x.0).1, x.1))
    }

    #[allow(unused)]
    fn tensor_id1<A>(x: Tensor<A, One>) -> A
    where
        A: 'static,
    {
        x.0
    }

    #[allow(unused)]
    fn tensor_id2<A>(x: A) -> Tensor<A, One>
    where
        A: 'static,
    {
        Tensor(x, One)
    }

    #[allow(unused)]
    fn tensor_id3<A>(x: Tensor<One, A>) -> A
    where
        A: 'static,
    {
        x.1
    }

    #[allow(unused)]
    fn tensor_id4<A>(x: A) -> Tensor<One, A>
    where
        A: 'static,
    {
        Tensor(One, x)
    }
}
