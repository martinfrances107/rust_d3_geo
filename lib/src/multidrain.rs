use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::path::context::Context;
use crate::path::string::String as PathString;
use crate::path::Result;

use crate::stream::Stream;

/// Stores a collection of sub drains used in the
/// `AbersUsa` pipeline.
#[derive(Clone, Debug)]
pub struct Multidrain<SD, T> {
    /// A collection of drains.
    pub drains: Vec<SD>,

    p_t: PhantomData<T>,
}

impl<SD, T> Default for Multidrain<SD, T> {
    fn default() -> Self {
        Self {
            drains: vec![],
            p_t: PhantomData::<T>,
        }
    }
}

impl<T> Result for Multidrain<Context, T> {
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

impl<T> Result for Multidrain<PathString<T>, T> {
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        vec![]
    }
}

impl<SD, T> Stream for Multidrain<SD, T>
where
    SD: Stream<EP = SD, T = T>,
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
