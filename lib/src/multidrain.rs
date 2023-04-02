use std::marker::PhantomData;

use geo::{Coord, CoordFloat};

use crate::path::context::Context;
use crate::path::Result;
use crate::projection::projector_albers_usa::multiplex::AlbersTransformer;
use crate::stream::Stream;

/// When asked for the results() output the contents of the Multiplex store in the
/// AbersUsa pipeline.
#[derive(Clone, Debug)]
pub struct Multidrain<EP, T, TRANSFORMER> {
    /// A collection of drains.
    pub drains: Vec<EP>,
    /// A collection fo transformer.
    pub transformers: Vec<TRANSFORMER>,
    p_t: PhantomData<T>,
}

impl<EP, T, TRANSFORMER> Default for Multidrain<EP, T, TRANSFORMER> {
    fn default() -> Self {
        Self {
            drains: vec![],
            transformers: vec![],
            p_t: PhantomData::<T>,
        }
    }
}

impl Result for Multidrain<Context, f64, AlbersTransformer<Context>> {
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        let mut out = vec![];
        for c in &mut self.drains {
            let results = c.result();
            for r in results {
                out.push(r);
            }
        }
        out
    }
}

impl Result for Multidrain<String, f64, AlbersTransformer<Context>> {
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        vec![]
    }
}

impl<EP, T> Stream for Multidrain<EP, T, AlbersTransformer<Context>>
where
    EP: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
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
