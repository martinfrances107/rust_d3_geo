use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::last_point::LastPoint;
use crate::path::string::String as PathString;
use crate::path::Result;
use crate::stream::Stream;

/// Wrapper for a Drain type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multidrain<const N: usize, SD, SUBTRANS, T> {
    p_t: PhantomData<T>,
    /// After initialisation, this value is used in a .connect()
    /// call to construct the drains.
    pub sd: SD,
    drains: Vec<SUBTRANS>,
}

impl<const N: usize, SD, T, SUBTRANS> Default for Multidrain<N, SD, SUBTRANS, T>
where
    SD: Default,
{
    fn default() -> Self {
        Self {
            p_t: PhantomData::<T>,
            sd: SD::default(),
            drains: vec![],
        }
    }
}

impl<const N: usize, SD, SUBTRANS, T> Multidrain<N, SD, SUBTRANS, T>
where
    SD: Clone + Default,
{
    /// Constructor.
    #[must_use]
    pub fn populate(&self, drains: Vec<SUBTRANS>) -> Self {
        Self {
            p_t: PhantomData::<T>,
            sd: self.sd.clone(),
            drains,
        }
    }
}

// impl<const N: usize, SUBTRANS, T> Result for Multidrain<N, PathString<T>, SUBTRANS, T>
// where
//     T: CoordFloat,
//     TRANSFORM: Result<Out = PathString<T>>,
// {
//     type Out = Vec<PathString<T>>;

//     /// Merges the results of all the sub-drains.
//     fn result(&mut self) -> Self::Out {
//         let mut out = vec![];
//         for c in &mut self.drains {
//             let result = c.result();
//             out.push(result);
//         }
//         out
//     }
// }

impl<const N: usize, SUBTRANS, T> Result for Multidrain<N, LastPoint<f64>, SUBTRANS, T>
where
    T: CoordFloat,
    SUBTRANS: Stream<EP = LastPoint<T>, T = T>,
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

impl<const N: usize, SD, SUBTRANS, T> Stream for Multidrain<N, SD, SUBTRANS, T>
where
    SUBTRANS: Stream<EP = SD, T = T>,
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
