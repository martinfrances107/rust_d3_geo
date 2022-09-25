use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Connectable;
use crate::Transform;

// pub mod types;
mod tests;
type CacheState<DRAIN, PCNC> = Option<(DRAIN, PCNC)>;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<DRAIN, PCNC, PCNU, T> {
    // Hidden linkage
    p_t: PhantomData<T>,
    /// Must be public as there is a implicit copy.
    pub(crate) postclip: PCNU,
    // pub(crate) rotator: RotatorRadians<Unconnected, T>,
    // projection: Transform<T = T>,
    // pub project_rotate_transform:
    //     Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,
    // pub(crate) transform_radians: StreamTransformRadians<Unconnected>,
    pub(crate) cache: CacheState<DRAIN, PCNC>,
}

// type ProjectionStream<T> = StreamTransformRadians<
//     Connected<RotatorRadians<Connected<Clip<I, LC, LU, PV, RC, ConnectedClip<LB, LC, T>, T>>, T>>,
// >;

impl<DRAIN, PCNC, PCNU, T> Projector<DRAIN, PCNC, PCNU, T>
where
    DRAIN: Clone + PartialEq,
    PCNC: Clone,
    PCNU: Clone + Connectable<SC = DRAIN, Output = PCNC>,
    PCNU: Clone,
    // T: CoordFloat,
{
    /// Connects a DRAIN to the projection.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// StreamTransformRadians -> StreamTransform -> preclip -> resample -> postclip -> DRAIN
    ///
    pub fn stream(&mut self, drain: &DRAIN) -> PCNC {
        if let Some((cache_drain, output)) = &self.cache {
            if *cache_drain == *drain {
                return (*output).clone();
            }
        }
        // Build cache.
        let out = self.postclip.clone().connect(drain.clone());

        // let resample_node = self.resample.clone().connect(postclip_node);

        // let preclip_node = self.clip.clone().connect(resample_node);

        // let rotate_node = self.rotator.clone().connect(preclip_node);

        // let out = self
        //     .transform_radians
        //     .clone()
        //     .connect::<DRAIN, _, T>(rotate_node);

        // Populate cache.
        self.cache = Some((drain.clone(), out.clone()));

        // Output stage is a transform_radians node.
        out
    }
}

impl<DRAIN, PCNC, PCNU, T> Transform for Projector<DRAIN, PCNC, PCNU, T>
where
    T: CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn transform(&self, _p: &Coordinate<T>) -> Coordinate<T> {
        // self.projecton.transform(p)
        todo!();
    }
    fn invert(&self, _p: &Coordinate<T>) -> Coordinate<T> {
        // self.projection.invert(p)
        todo!();
    }
}
