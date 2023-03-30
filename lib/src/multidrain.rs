use std::marker::PhantomData;

use geo::{Coord, CoordFloat};

use crate::path::context::Context;
use crate::path::Result;
use crate::stream::Stream;

// When asked for the results() output the contents of the Multiplex store in the
// AbersUsa pipeline.
#[derive(Debug)]
pub struct Multidrain<EP, T> {
    store: Vec<EP>,
    p_t: PhantomData<T>,
}

impl Result for Multidrain<Context, f64> {
    type Out = Vec<String>;

    /// Returns current the end points calculation.
    fn result(&mut self) -> Self::Out {
        let mut out = vec![];
        for c in &mut self.store {
            let results = c.result();
            for r in results {
                out.push(r);
            }
        }
        out
    }
}

impl Result for Multidrain<String, f64> {
    type Out = Vec<String>;

    /// Returns current the end points calculation.
    fn result(&mut self) -> Self::Out {
        vec![]
    }
}

impl<EP, T> Stream for Multidrain<EP, T>
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
