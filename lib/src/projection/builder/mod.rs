use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::clip::Clip;
use crate::compose::Compose;
use crate::identity::Identity;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneNoClip;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleNoneNoClip;
use crate::projection::RecenterWithResampling;
use crate::rot::rotate_radians;
use crate::rot::rotate_radians::RotateRadians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::builder::template::NoClipC;
use super::builder::template::NoClipU;
use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;
use super::resampler::resample::Resample;
use super::transform::generate as generate_str;
use super::transform::scale_translate_rotate::ScaleTranslateRotate;

use template::ClipC;
use template::ClipU;
use template::ResampleClipC;
use template::ResampleClipU;

mod angle;
mod angle_get;
mod build;
mod center_get;
mod center_set;
mod clip_angle_adjust;
mod clip_angle_get;
mod clip_angle_reset;
mod clip_angle_set;
mod clip_bounded;
mod clip_extent_adjust;
mod clip_extent_set;
mod fit_adjust;
mod fit_no_clip;
mod precision_adjust;
mod precision_bypass;
mod precision_get;
mod precision_set;
mod recenter;
mod reflect_get;
mod reflect_set;
mod rotate;
mod rotate_get;
mod scale_adjust;
mod scale_get;
pub mod template;
mod translate_adjust;
mod translate_get;
pub mod types;

/// Marker trait for structs Identity or Rectangle
pub trait PostClipNode {}

/// Projection builder.
///
/// Holds State related to the construction of the a projection.
#[derive(Clone, Debug)]
pub struct Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: CoordFloat,
{
    pub p_lb: PhantomData<LB>,
    pub p_pcnc: PhantomData<PCNC>,
    pub p_drain: PhantomData<DRAIN>,
    pub projection_raw: PR,
    pub clip: Clip<I, LC, LU, PV, RC, RU, Unconnected, T>,
    pub lambda: T,
    pub phi: T,
    pub alpha: T, // post-rotate angle
    pub k: T,     // scale
    pub sx: T,    // reflectX
    pub sy: T,    // reflectY

    pub x: T,
    pub y: T, // translate

    pub delta_lambda: T,
    pub delta_phi: T,
    pub delta_gamma: T,

    pub delta2: T, // precision

    pub theta: Option<T>,

    pub x0: Option<T>,
    pub y0: Option<T>,
    pub x1: Option<T>,
    pub y1: Option<T>, // post-clip extent

    /// Used by recenter() to build the factories.
    pub rotate: RotateRadians<T>,
    pub rotator: RotatorRadians<Unconnected, T>, //rotate, pre-rotate
    pub project_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
    /// Used by rotate_transform_factory and projections transform.
    pub project_rotate_transform:
        Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,

    /// Projection pipeline stage.
    pub postclip: PCNU,

    /// Projection pipeline stage
    pub resample: RU,
}

impl<DRAIN, PR, T> BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    Self: RecenterWithResampling,
    T: CoordFloat + FloatConst,
{
    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    pub fn new(
        clip: Clip<
            InterpolateAntimeridian<T>,
            LineAntimeridian<
                ResampleNoClipC<DRAIN, PR, T>,
                Connected<ResampleNoClipC<DRAIN, PR, T>>,
                T,
            >,
            LineAntimeridian<ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
            PVAntimeridian<T>,
            ResampleNoClipC<DRAIN, PR, T>,
            ResampleNoClipU<DRAIN, PR, T>,
            Unconnected,
            T,
        >,
        projection_raw: PR,
    ) -> Self {
        let x = T::from(480_f64).unwrap();
        let y = T::from(250_f64).unwrap();
        let lambda = T::zero();
        let phi = T::zero();
        let alpha = T::zero();
        let k = T::from(150_f64).unwrap();
        let sx = T::one();
        let sy = T::one();
        let delta_lambda = T::zero();
        let delta_phi = T::zero();
        let delta_gamma = T::zero();
        let delta2 = T::from(0.5_f64).unwrap();
        let center = generate_str(&k, &T::zero(), &T::zero(), &sx, &sy, &alpha)
            .transform(&projection_raw.transform(&Coordinate { x: lambda, y: phi }));
        let str = generate_str(&k, &(x - center.x), &(y - center.y), &sx, &sy, &alpha);

        let rotate = rotate_radians([delta_lambda, delta_phi, delta_gamma]); // pre-rotate
        let rotator = RotatorRadians::new(rotate.clone());
        let project_transform = Compose::new(projection_raw.clone(), str);
        let project_rotate_transform = Compose::new(rotate.clone(), project_transform.clone());
        let postclip = Identity::default();
        let resample = Resample::new(project_transform.clone(), delta2);
        let p_pcnc = PhantomData::<NoClipC<DRAIN>>;
        let out_a: Self = Self {
            clip,
            p_lb: PhantomData::<LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>>,
            p_pcnc,
            p_drain: PhantomData::<DRAIN>,
            /// Input passing onto Projection.
            projection_raw,

            /// Internal state.
            delta_lambda,
            delta_phi,
            delta_gamma,
            x,
            y,

            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent

            delta2: T::from(0.5_f64).unwrap(),
            lambda,
            phi,

            alpha,
            k,
            theta: None,
            sx,
            sy,

            rotate,
            rotator,
            project_transform,
            project_rotate_transform,
            postclip,
            resample,
        };

        let out_b: Self = out_a.recenter_with_resampling();
        out_b
    }
}
