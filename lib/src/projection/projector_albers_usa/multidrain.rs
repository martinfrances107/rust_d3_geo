use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::last_point::LastPoint;
use crate::path::string::String as PathString;
use crate::path::Result;
use crate::stream::Stream;

/// Only when the Multidrain is connected the sub drain becomes known
/// as so the SUBTRANS type can be defined.
#[derive(Clone, Debug)]
pub struct Populated<SUBTRANS> {
    drains: Vec<SUBTRANS>,
}

/// The state before connection and the drain is populated
#[derive(Debug)]
pub struct Unpopulated;

/// Wrapper for a Drain type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multidrain<const N: usize, SD, STATE, T> {
    p_t: PhantomData<T>,
    /// After initialisation, this value is used in a .connect()
    /// call to construct the drains.
    pub sd: SD,
    // drains: Vec<SUBTRANS>,
    state: STATE,
}

impl<const N: usize, SD, T> Multidrain<N, SD, Unpopulated, T> {
    pub const fn new(sd: SD) -> Self {
        Self {
            p_t: PhantomData::<T>,
            sd,
            state: Unpopulated,
        }
    }
}

impl<const N: usize, SD, T> Multidrain<N, SD, Unpopulated, T>
where
    SD: Clone + Default,
{
    /// Constructor.
    #[must_use]
    pub fn populate<SUBTRANS>(
        &self,
        drains: Vec<SUBTRANS>,
    ) -> Multidrain<N, SD, Populated<SUBTRANS>, T> {
        Multidrain {
            p_t: PhantomData::<T>,
            sd: self.sd.clone(),
            state: Populated { drains },
        }
    }
}

impl<const N: usize, SUBTRANS, T> Result for Multidrain<N, PathString<T>, Populated<SUBTRANS>, T>
where
    SUBTRANS: Result<Out = Vec<PathString<T>>>,
    T: CoordFloat,
{
    type Out = Vec<PathString<T>>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        let mut out = vec![];
        for c in &mut self.state.drains {
            let results = c.result();
            for result in results {
                out.push(result);
            }
        }
        out
    }
}

impl<const N: usize, SUBTRANS, T> Result for Multidrain<N, LastPoint<f64>, Populated<SUBTRANS>, T>
where
    T: CoordFloat,
    SUBTRANS: Stream<EP = LastPoint<T>, T = T>,
{
    type Out = Option<Coord<T>>;
    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        for d in &mut self.state.drains {
            if let Some(p) = d.endpoint().result() {
                return Some(p);
            }
        }
        None
    }
}

impl<const N: usize, SD, SUBTRANS, T> Stream for Multidrain<N, SD, Populated<SUBTRANS>, T>
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
        for item in &mut self.state.drains {
            item.line_end();
        }
    }

    fn line_start(&mut self) {
        for item in &mut self.state.drains {
            item.line_start();
        }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.state.drains {
            item.point(p, m);
        }
    }

    fn polygon_end(&mut self) {
        for item in &mut self.state.drains {
            item.polygon_end();
        }
    }

    fn polygon_start(&mut self) {
        for item in &mut self.state.drains {
            item.polygon_start();
        }
    }

    fn sphere(&mut self) {
        for item in &mut self.state.drains {
            item.sphere();
        }
    }
}
