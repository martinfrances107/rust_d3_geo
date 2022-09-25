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
    p_drain: PhantomData<DRAIN>,
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
    #[inline]
    pub fn default() -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,

            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent

            alpha: T::zero(),
            k: T::one(),
            kx: T::one(),
            ky: T::one(),
            ca: T::nan(),
            sa: T::nan(),
            sx: T::one(),
            sy: T::one(),
            tx: T::zero(),
            ty: T::zero(),
            postclip: Identity::default(),
        }
    }
}

impl<DRAIN, PCNU, T> Builder<DRAIN, PCNU, T>
where
    T: CoordFloat,
{
    fn reset(mut self) -> Self {
        self.kx = self.k * self.sx;
        self.ky = self.k * self.sy;
        self
    }
}
