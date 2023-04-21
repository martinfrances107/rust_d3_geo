use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::last_point::LastPoint;
use crate::path::string::String as PathString;
use crate::path::Result;
use crate::stream::Stream;

/// Wrapper for a Drain type.
#[derive(Clone, Debug, PartialEq)]
pub struct Multidrain<const N: usize, SD, T, TRANSFORMER> {
    p_t: PhantomData<T>,
    pub sd: SD,
    drains: Vec<TRANSFORMER>,
}

impl<const N: usize, SD, T, TRANSFORMER> Default for Multidrain<N, SD, T, TRANSFORMER>
where
    SD: Default,
{
    fn default() -> Self {
        todo!();
        // Self {
        //     p_t: PhantomData::<T>,
        //     sd: SD::default(),
        //     drains: vec![],
        // }
    }
}

impl<const N: usize, SD, T, TRANSFORMER> Multidrain<N, SD, T, TRANSFORMER>
where
    SD: Default,
{
    pub fn new(drains: [TRANSFORMER; N]) -> Self {
        todo!();
        // Self {
        //     p_t: PhantomData::<T>,
        //     sd: SD::default(),
        //     drains: vec![],
        // }
    }
}

impl<const N: usize, T, TRANSFORMER> Result for Multidrain<N, PathString<T>, T, TRANSFORMER>
where
    T: CoordFloat,
{
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        todo!();
        // let mut out = vec![];
        // for c in &mut self.drains {
        //     let result = c.result();
        //     out.push(result);
        // }
        // out
    }
}

impl<const N: usize, T, TRANSFORMER> Result for Multidrain<N, LastPoint<f64>, T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Stream<EP = LastPoint<T>, T = T>,
{
    type Out = Option<Coord<T>>;
    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        for d in &mut self.drains {
            if let Some(p) = d.endpoint().result() {
                return Some(p);
            }
        }
        None
    }
}

impl<const N: usize, SD, T, TRANSFORMER> Stream for Multidrain<N, SD, T, TRANSFORMER>
where
    SD: Stream<EP = SD, T = T>,
    TRANSFORMER: Stream<EP = SD, T = T>,
    T: CoordFloat,
{
    type T = T;
    type EP = Self;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    fn line_end(&mut self) {
        for item in &mut self.drains {
            item.line_end();
        }
    }

    fn line_start(&mut self) {
        for item in &mut self.drains {
            item.line_start();
        }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.drains {
            item.point(p, m);
        }
    }

    fn polygon_end(&mut self) {
        for item in &mut self.drains {
            item.polygon_end();
        }
    }

    fn polygon_start(&mut self) {
        for item in &mut self.drains {
            item.polygon_start();
        }
    }

    fn sphere(&mut self) {
        for item in &mut self.drains {
            item.sphere();
        }
    }
}
