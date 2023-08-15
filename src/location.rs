use std::fmt::Debug;
use std::ops::{Deref, FromResidual, Try};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Locational<T> {
    pub loc: Location,
    pub t: T,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl<T> Locational<T> {
    pub fn from_loc(loc: Location, t: T) -> Self {
        Self { loc, t }
    }

    pub fn from(start: usize, end: usize, t: T) -> Self {
        Self::from_loc(Location::new(start, end), t)
    }

    pub fn ok<RT, RE>(self) -> Locational<Result<RT, RE>>
    where
        T: Into<RT>,
    {
        let Locational { loc, t } = self;

        return Locational::from_loc(loc, Ok(t.into()));
    }

    pub fn err<RT, RE>(self) -> Locational<Result<RT, RE>>
    where
        T: Into<RE>,
    {
        let Locational { loc, t } = self;

        return Locational::from_loc(loc, Err(t.into()));
    }
}

impl<T> Deref for Locational<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T, E> Try for Locational<Result<T, E>> {
    type Output = Locational<T>;

    type Residual = Locational<E>;

    fn from_output(output: Self::Output) -> Self {
        Locational::from_loc(output.loc, Ok(output.t))
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        let Self { loc, t } = self;

        match t {
            Ok(val) => std::ops::ControlFlow::Continue(Self::Output { loc, t: val }),
            Err(err) => std::ops::ControlFlow::Break(Self::Residual { loc, t: err }),
        }
    }
}

impl<T, E1, E2> FromResidual<Locational<E1>> for Locational<Result<T, E2>>
where
    E2: std::convert::From<E1>,
{
    fn from_residual(residual: Locational<E1>) -> Self {
        residual.err()
    }
}
