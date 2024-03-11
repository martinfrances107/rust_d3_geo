use geo::Coord;
use geo::CoordFloat;

use crate::last_point::LastPoint;
use crate::path::string::String as PathString;
use crate::path::Result;
use crate::stream::Stream;

use super::AlbersTransformer;

/// Only when the Multidrain is connected the sub drain becomes known
/// as so the SUBTRANS type can be defined.
#[derive(Clone, Debug)]
pub struct Populated<const N: usize, SUBTRANS> {
    store: [SUBTRANS; N],
}

/// The state before connection and the drain is populated
#[derive(Debug)]
pub struct Unpopulated;

/// Wrapper for a Drain type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multidrain<const N: usize, SD, STATE> {
    /// After initialisation, this value is used in a `.connect()`
    /// call to construct the drains.
    pub sd: SD,
    state: STATE,
}

impl<const N: usize, SD> Multidrain<N, SD, Unpopulated> {
    /// Define initial multidrain.
    /// [`Multidrain::populate()`] must be called to complete
    // initialisation.
    pub const fn new(sd: SD) -> Self {
        Self {
            sd,
            state: Unpopulated,
        }
    }
}

impl<const N: usize, SD> Multidrain<N, SD, Unpopulated>
where
    SD: Clone,
{
    /// Provide transforms associated with each inset.
    // ( This complete initialisation )
    #[must_use]
    pub fn populate<SUBTRANS>(
        &self,
        store: [SUBTRANS; N],
    ) -> Multidrain<N, SD, Populated<N, SUBTRANS>> {
        Multidrain {
            sd: self.sd.clone(),
            state: Populated { store },
        }
    }
}

impl Result
    for Multidrain<
        3,
        PathString<f64>,
        Populated<3, AlbersTransformer<PathString<f64>, f64>>,
    >
{
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        let mut out = vec![];

        for c in &mut self.state.store {
            let results = c.endpoint().result();

            out.push(results);
        }
        out
    }
}

impl<const N: usize, SUBTRANS, T> Result
    for Multidrain<N, LastPoint<f64>, Populated<N, SUBTRANS>>
where
    SUBTRANS: Stream<EP = LastPoint<T>, T = T>,
    T: CoordFloat,
{
    type Out = Option<Coord<T>>;
    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        for d in &mut self.state.store {
            if let Some(p) = d.endpoint().result() {
                return Some(p);
            }
        }
        None
    }
}

impl<const N: usize, SD, SUBTRANS, T> Stream
    for Multidrain<N, SD, Populated<N, SUBTRANS>>
where
    SUBTRANS: Stream<EP = SD, T = T>,
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    fn line_end(&mut self) {
        for item in &mut self.state.store {
            item.line_end();
        }
    }

    fn line_start(&mut self) {
        for item in &mut self.state.store {
            item.line_start();
        }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.state.store {
            item.point(p, m);
        }
    }

    fn polygon_end(&mut self) {
        for item in &mut self.state.store {
            item.polygon_end();
        }
    }

    fn polygon_start(&mut self) {
        for item in &mut self.state.store {
            item.polygon_start();
        }
    }

    fn sphere(&mut self) {
        for item in &mut self.state.store {
            item.sphere();
        }
    }
}
