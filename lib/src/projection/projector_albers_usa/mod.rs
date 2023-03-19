use geo::CoordFloat;
use std::fmt::Debug;

use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected;
use crate::stream::Stream;

use super::albers_usa::AlbersUsa;
use super::builder_conic::types::BuilderConicAntimeridianResampleClip;
use super::stream_transform_radians::StreamTransformRadians;

mod multiplex;
/// Builder shorthand notations.
pub mod types;

use multiplex::Multiplex;

type CacheState<CLIP, DRAIN, T> = Option<(
    DRAIN,
    StreamTransformRadians<Connected<RotatorRadians<Connected<CLIP>, T>>>,
)>;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<CLIPC, DRAIN, T>
where
    DRAIN: Clone,
    CLIPC: Clone,
    T: CoordFloat,
{
    // pub(crate) p_rc: PhantomData<RC>,
    multiplex: Multiplex<
        DRAIN,
        3,
        BuilderConicAntimeridianResampleClip<DRAIN, AlbersUsa<DRAIN>, f64>,
        f64,
    >,
    pub(crate) cache: CacheState<CLIPC, DRAIN, T>,
}

type ProjectionStream<CLIP, T> =
    StreamTransformRadians<Connected<RotatorRadians<Connected<CLIP>, T>>>;

// impl<CC, DRAIN, T> Projector<CC, DRAIN, T>
// where
//     CC: Clone + Stream<EP = DRAIN, T = T>,
//     DRAIN: Clone + PartialEq,
//     // PCNC: Clone,
//     // RU: Clone + Connectable<Output<PCNC> = RC>,
//     // RC: Clone,
//     T: CoordFloat,
// {
//     /// Connects a DRAIN to the projection.
//     ///
//     /// The Projection Stream Pipeline :-
//     ///
//     /// `StreamTransformRadians` -> `StreamTransform` -> `Preclip` -> `Resample` -> `Postclip` -> `DRAIN`
//     ///
//     pub fn stream(&mut self, drain: &DRAIN) -> ProjectionStream<CC, T> {
//         // if let Some((cache_drain, output)) = &self.cache {
//         //     if *cache_drain == *drain {
//         //         return (*output).clone();
//         //     }
//         // }
//         // // Build cache.
//         // let postclip_node = self.postclip.clone().connect(drain.clone());

//         self.multiplex.clone().connect(drain.clone())
//     }
// }

// impl<CLIPC, DRAIN, PCNU, PR, RC, RU, T> Transform for Projector<CLIPC, DRAIN, PR, RC, RU, T>
// where
//     CLIPC: Clone,

//     PR: Transform<T = T>,
//     T: CoordFloat + FloatConst,
// {
//     /// f32 or f64
//     type T = T;

//     fn transform(&self, p: &Coord<T>) -> Coord<T> {
//         todo!();
//         // let r = Coord {
//         //     x: p.x.to_radians(),
//         //     y: p.y.to_radians(),
//         // };
//         // self.project_rotate_transform.transform(&r)
//     }
//     fn invert(&self, p: &Coord<T>) -> Coord<T> {
//         todo!();
//         // let d = self.project_rotate_transform.invert(p);
//         // Coord {
//         //     x: d.x.to_degrees(),
//         //     y: d.y.to_degrees(),
//         // }
//     }
// }
