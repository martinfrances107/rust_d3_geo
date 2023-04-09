use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;

use crate::last_point::LastPoint;
use crate::path::context::Context;
use crate::path::string::String as PathString;
use crate::path::Result;
use crate::stream::Connectable;
use crate::stream::Stream;

/// Projections like `AlbersUSA` group several projections together.
#[derive(Debug)]
pub struct MultiTransformer<const N: usize, SD, TRANSFORMER> {
    p_sd: PhantomData<SD>,
    // state: STATE,
    store: [TRANSFORMER; N],
}

impl<const N: usize, SD, TRANSFORMER> MultiTransformer<N, SD, TRANSFORMER>
where
    TRANSFORMER: Stream<EP = SD, T = f64>,
{
    /// Constructor
    pub const fn new(store: [TRANSFORMER; N]) -> Self {
        Self {
            p_sd: PhantomData::<SD>,
            // state: Connected { sink },
            store,
        }
    }
}

impl<const N: usize, SD, TRANSFORMER> Connectable for MultiTransformer<N, SD, TRANSFORMER>
where
    TRANSFORMER: Clone + Stream<EP = SD, T = f64>,
{
    type Output<SC: Clone> = Self;

    #[inline]
    fn connect<SC: Clone>(&self, _sink: SC) -> Self::Output<SC> {
        Self {
            p_sd: PhantomData::<SD>,
            store: self.store.clone(),
        }
    }
}

impl<const N: usize, TRANSFORMER> Result for MultiTransformer<N, Context, TRANSFORMER>
where
    TRANSFORMER: Stream<EP = Context, T = f64>,
{
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        let mut out = vec![];
        for c in &mut self.store {
            let results = c.endpoint().result();
            for r in results {
                out.push(r);
            }
        }
        out
    }
}

impl<const N: usize, T, TRANSFORMER> Result for MultiTransformer<N, PathString<T>, TRANSFORMER>
where
    TRANSFORMER: Stream<EP = PathString<T>, T = f64>,
{
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        vec![]
    }
}

impl<const N: usize, TRANSFORMER> Result for MultiTransformer<N, LastPoint<f64>, TRANSFORMER>
where
    TRANSFORMER: Stream<EP = LastPoint<f64>, T = f64>,
{
    type Out = Option<Coord<f64>>;
    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        for d in &mut self.store {
            if let Some(p) = d.endpoint().result() {
                return Some(p);
            }
        }
        None
    }
}

impl<const N: usize, SD, TRANSFORMER> Stream for MultiTransformer<N, SD, TRANSFORMER>
where
    SD: Stream<EP = SD, T = f64> + Debug,
    TRANSFORMER: Stream<EP = SD, T = f64>,
{
    type T = f64;
    type EP = Self;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    fn line_end(&mut self) {
        for item in &mut self.store {
            item.line_end();
        }
    }

    fn line_start(&mut self) {
        for item in &mut self.store {
            item.line_start();
        }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.store {
            item.point(p, m);
        }
    }

    fn polygon_end(&mut self) {
        for item in &mut self.store {
            item.polygon_end();
        }
    }

    fn polygon_start(&mut self) {
        for item in &mut self.store {
            item.polygon_start();
        }
    }

    fn sphere(&mut self) {
        for item in &mut self.store {
            item.sphere();
        }
    }
}
