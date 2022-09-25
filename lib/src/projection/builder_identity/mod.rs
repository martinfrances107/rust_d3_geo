use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::identity::Identity;

use crate::stream::Unconnected;

mod angle;
mod angle_get;
mod build;

// mod fit_clip;
// mod fit_no_clip;
mod clip_extent_adjust;
mod clip_extent_clear;
mod clip_extent_get;
mod clip_extent_set;
mod reflect_get;
mod reflect_set;
mod scale_get;
mod scale_set;
mod translate_get;
mod translate_set;

// pub mod template;
// pub mod types;
/// Projection builder.
///
/// Holds State related to the construction of the a projection.
#[derive(Clone, Debug)]
pub struct Builder<DRAIN, PCNU, T>
where
    T: CoordFloat,
{
    /// PhantomData<LB>
    /// The hidden link is between the Projector<..,LB,..>
    /// and the builder.    
    p_drain: PhantomData<DRAIN>,
    // projection_raw: PR,
    pub(super) alpha: T, // post-rotate angle
    pub(super) ca: T,
    pub(super) sa: T,
    pub(super) sx: T,
    pub(super) sy: T,
    pub(super) k: T,
    pub(super) kx: T, // scale
    pub(super) ky: T,

    pub(super) tx: T,
    pub(super) ty: T, // translate

    x0: Option<T>,
    y0: Option<T>,
    x1: Option<T>,
    y1: Option<T>, // post-clip extent

    /// Projection pipeline stage.
    pub(super) postclip: PCNU,
}

impl<DRAIN, T> Builder<DRAIN, Identity<DRAIN, Unconnected>, T>
where
    T: CoordFloat + Default + FloatConst,
{
    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    pub fn default() -> Self {
        // let x = T::from(480_f64).unwrap();
        // let y = T::from(250_f64).unwrap();

        let alpha = T::zero();

        let postclip = Identity::default();
        Self {
            // clip: gen_clip_antimeridian::<NoClipU<DRAIN>, _, _>(),
            // p_lb: PhantomData::<LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>>,
            p_drain: PhantomData::<DRAIN>,

            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent
            ca: T::nan(),

            alpha,
            k: T::one(),
            kx: T::one(),
            ky: T::one(),
            sa: T::nan(),
            sx: T::one(),
            sy: T::one(),
            tx: T::zero(),
            ty: T::zero(),
            postclip,
            // project_transform,
            // project_rotate_transform,
        }
    }
}

impl<DRAIN, PCNU, T> Builder<DRAIN, PCNU, T>
where
    T: CoordFloat,
{
    fn reset(&mut self) {
        self.kx = self.k * self.sx;
        self.ky = self.k * self.sy;
    }
}
