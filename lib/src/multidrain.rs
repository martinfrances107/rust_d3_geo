use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::last_point::LastPoint;
use crate::path::context::Context;
use crate::path::string::String as PathString;
use crate::path::Result;

use crate::stream::Stream;

/// Stores a collection of sub drains used in the
/// `AbersUsa` pipeline.
#[derive(Clone, Debug)]
pub struct Multidrain<const N: usize, SD, T> {
    /// A collection of drains.
    pub drains: [SD; N],

    p_t: PhantomData<T>,
}

/// TODO: At the moment `AlbersUSA` needs only
/// 3-element `MultiTransformer`'s, 3-element `Multidrains`'s
///
/// When I need to, I should make this more generic.
impl<SD, T> Default for Multidrain<3usize, SD, T>
where
    SD: Default,
{
    fn default() -> Self {
        Self {
            drains: [SD::default(), SD::default(), SD::default()],
            p_t: PhantomData::<T>,
        }
    }
}

impl<const N: usize, T> Result for Multidrain<N, Context, T> {
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

impl<const N: usize, T> Result for Multidrain<N, PathString<T>, T> {
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        vec![]
    }
}

impl<const N: usize, T> Result for Multidrain<N, LastPoint<T>, T>
where
    T: CoordFloat,
{
    type Out = Option<Coord<T>>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        for d in &mut self.drains {
            if let Some(p) = d.result() {
                return Some(p);
            }
        }
        None
    }
}

impl<const N: usize, SD, T> Stream for Multidrain<N, SD, T>
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
        dbg!("Multidrain {}", p);
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
