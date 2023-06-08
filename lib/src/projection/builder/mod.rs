use core::fmt::Debug;
use core::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip;
use crate::compose::Compose;
use crate::identity::Identity;
use crate::projection::builder::template::ResampleNoneNoPCNU;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneNoClip;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleNoneNoClip;
use crate::projection::Recenter;
use crate::rot::rotate_radians;
use crate::rot::rotate_radians::RotateRadians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Unconnected;
use crate::Transform;

use super::builder::template::ResampleNoPCNC;
use super::builder::template::ResampleNoPCNU;
use super::resampler::resample::Resample;
use super::transform::generate as generate_str;
use super::transform::scale_translate_rotate::ScaleTranslateRotate;
use super::BuilderTrait;

use template::ResamplePCNC;
use template::ResamplePCNU;

mod angle;
mod angle_get;
mod build;
mod center_get;
mod center_set;
mod clip_angle_adjust;
mod clip_angle_get;
mod clip_angle_reset;
mod clip_angle_set;
mod clip_extent_adjust;
mod clip_extent_clear;
mod clip_extent_get;
mod clip_extent_set;
mod fit_clip;
mod fit_no_clip;
mod precision_adjust;
mod precision_bypass;
mod precision_get;
mod precision_set;
mod recenter_no_resampling;
mod recenter_with_resampling;
mod reflect_get;
mod reflect_set;
mod rotate_get;
mod rotate_set;
mod scale_get;
mod scale_no_resampling;
mod scale_with_resampling;
mod transform;
mod translate_get;
mod translate_no_resampling;
mod translate_with_resampling;

pub mod template;
/// Builder shorthand notations.
pub mod types;

/// Projection builder state.
#[derive(Clone, Debug)]
pub struct Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    p_d: PhantomData<DRAIN>,
    pub(super) projection_raw: PR,
    pub(super) clip: CLIPU,
    lambda: T,
    phi: T,
    alpha: T, // post-rotate angle
    k: T,     // scale
    sx: T,    // reflectX
    sy: T,    // reflectY

    x: T,
    y: T, // translate

    t360: T,
    delta_lambda: T,
    delta_phi: T,
    delta_gamma: T,

    delta2: T, // precision

    theta: Option<T>,

    /// Used by recenter() to build the factories.
    rotate: RotateRadians<T>,
    pub(super) rotator: RotatorRadians<Unconnected, T>, //rotate, pre-rotate
    project_transform: Compose<PR, ScaleTranslateRotate<T>>,
    /// Used by rotate_transform_factory and projections transform.
    pub(super) project_rotate_transform:
        Compose<RotateRadians<T>, Compose<PR, ScaleTranslateRotate<T>>>,

    /// Projection path stage.
    pub(super) postclip: PCNU,

    /// Projection path stage
    pub(super) resample: RU,
}

impl<DRAIN, PR, T> BuilderTrait for BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
where
    PR: Clone + Transform<T = T>,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type PR = PR;

    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as constants will always be converted into T.
    fn new(projection_raw: PR) -> Self {
        let x = T::from(480_f64).unwrap();
        let y = T::from(250_f64).unwrap();
        let lambda = T::zero();
        let phi = T::zero();
        let alpha = T::zero();
        let k = T::from(150_f64).unwrap();
        let sx = T::one();
        let sy = T::one();
        let t360 = T::from(360_f64).unwrap();
        let delta_lambda = T::zero();
        let delta_phi = T::zero();
        let delta_gamma = T::zero();
        let delta2 = T::from(0.5_f64).unwrap();
        let center = generate_str(&k, &T::zero(), &T::zero(), &sx, &sy, &alpha)
            .transform(&projection_raw.transform(&Coord { x: lambda, y: phi }));
        let str = generate_str(&k, &(x - center.x), &(y - center.y), &sx, &sy, &alpha);

        let rotate = rotate_radians([delta_lambda, delta_phi, delta_gamma]); // pre-rotate
        let rotator = RotatorRadians::new(rotate.clone());
        let project_transform = Compose::new(projection_raw.clone(), str);
        let project_rotate_transform = Compose::new(rotate.clone(), project_transform.clone());
        let postclip = Identity::default();
        let resample = Resample::new(project_transform.clone(), delta2);
        let mut out: Self = Self {
            p_d: PhantomData::<DRAIN>,
            clip: gen_clip(),
            /// Input passing onto Projection.
            projection_raw,

            /// Internal state.
            delta_lambda,
            delta_phi,
            delta_gamma,
            x,
            y,
            t360,

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

        out.recenter();
        out
    }
}
