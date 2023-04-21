use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::stream::Stream;

use super::multidrain::Multidrain;

/// Projections like `AlbersUSA` group several projections together.
#[derive(Debug)]
pub struct MultiTransformer<const N: usize, SD, T, TRANSFORMER> {
    p_t: PhantomData<T>,
    p_sd: PhantomData<SD>,
    // store: [TRANSFORMER; N],
    // The contained endpoint.
    pub md: Multidrain<N, SD, T, TRANSFORMER>,
}

impl<const N: usize, SD, T, TRANSFORMER> MultiTransformer<N, SD, T, TRANSFORMER>
where
    SD: Default,
{
    /// Constructor
    pub fn new(store: [TRANSFORMER; N]) -> Self {
        Self {
            p_t: PhantomData::<T>,
            p_sd: PhantomData::<SD>,
            md: Multidrain::new(store),
        }
    }
}

// impl<const N: usize, SD, T, TRANSFORMER> Connectable for MultiTransformer<N, SD, T, TRANSFORMER>
// where
//     TRANSFORMER: Clone,
// {
//     type Output<SC: Clone> = Self;

//     #[inline]
//     fn connect<SC: Clone>(&self, _sink: SC) -> Self::Output<SC> {
//         Self {
//             p_t: PhantomData::<T>,
//             p_sd: PhantomData::<SD>,
//             store: self.store.clone(),
//         }
//     }
// }

// impl<const N: usize, T, TRANSFORMER> Result for MultiTransformer<N, Context, T, TRANSFORMER>
// where
//     TRANSFORMER: Stream<EP = Context, T = T>,
// {
//     type Out = Vec<String>;

//     /// Merges the results of all the sub-drains.
//     fn result(&mut self) -> Self::Out {
//         let mut out = vec![];
//         for c in &mut self.drains {
//             let results = c.endpoint().result();
//             for r in results {
//                 out.push(r);
//             }
//         }
//         out
//     }
// }

// impl<const N: usize, T, TRANSFORMER> Result for MultiTransformer<N, PathString<T>, T, TRANSFORMER>
// where
//     T: CoordFloat,
//     TRANSFORMER: Stream<EP = PathString<T>, T = T>,
// {
//     type Out = Vec<String>;

//     /// Merges the results of all the sub-drains.
//     fn result(&mut self) -> Self::Out {
//         let mut out = vec![];
//         for c in &mut self.store {
//             let result = c.endpoint().result();
//             out.push(result);
//         }
//         out
//     }
// }

// impl<const N: usize, T, TRANSFORMER> Result for MultiTransformer<N, LastPoint<f64>, T, TRANSFORMER>
// where
//     T: CoordFloat + Debug,
//     TRANSFORMER: Stream<EP = LastPoint<T>, T = T>,
// {
//     type Out = Option<Coord<T>>;
//     /// Merges the results of all the sub-drains.
//     fn result(&mut self) -> Self::Out {
//         for d in &mut self.store {
//             if let Some(p) = d.endpoint().result() {
//                 return Some(p);
//             }
//         }
//         None
//     }
// }

// TODO can I remove this wrapper.
impl<const N: usize, SD, T, TRANSFORMER> Stream for MultiTransformer<N, SD, T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Stream<EP = SD, T = T>,
    SD: Stream<EP = SD, T = T>,
{
    type T = T;
    type EP = Self;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    fn line_end(&mut self) {
        self.md.line_end();
    }

    fn line_start(&mut self) {
        self.md.line_start();
        // for item in &mut self.store {
        //     item.line_start();
        // }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        // for item in &mut self.md.drains {
        //     item.point(p, m);
        // }
        self.md.point(p, m);
    }

    fn polygon_end(&mut self) {
        // for item in &mut self.store {
        //     item.polygon_end();
        // }
        self.md.polygon_end();
    }

    fn polygon_start(&mut self) {
        // for item in &mut self.store {
        //     item.polygon_start();
        // }
        self.md.polygon_start();
    }

    fn sphere(&mut self) {
        // for item in &mut self.store {
        //     item.sphere();
        // }
        self.md.sphere();
    }
}
