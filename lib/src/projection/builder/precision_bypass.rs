use core::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip as gen_clip_antimeridian;
use crate::clip::circle::gen_clip as gen_clip_circle;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneClip;
use crate::projection::resampler::none::None;
use crate::projection::PrecisionBypass;

use super::template::ResampleNoneNoPCNC;
use super::template::ResampleNonePCNC;
use super::types::BuilderAntimeridianResampleClip;
use super::types::BuilderAntimeridianResampleNoClip;
use super::types::BuilderAntimeridianResampleNoneNoClip;
use super::types::BuilderCircleResampleClip;
use super::types::BuilderCircleResampleNoClip;
use super::types::BuilderCircleResampleNoneClip;
use super::types::BuilderCircleResampleNoneNoClip;

impl<DRAIN, PR, T> PrecisionBypass
    for BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type Output = BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T>;
    type T = T;

    // Switch the builder into one without a resample node,
    #[inline]
    fn precision_bypass(&self) -> Self::Output {
        // Copy - Mutate.
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            t360: self.t360,
            theta: self.theta,
            rotate: self.rotate.clone(),
            rotator: self.rotator.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            postclip: self.postclip.clone(),
            alpha: self.alpha,
            lambda: self.lambda,
            phi: self.phi,
            projection_raw: self.projection_raw.clone(),
            k: self.k,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,

            // Mutate section.
            clip: gen_clip_antimeridian::<ResampleNoneNoPCNC<DRAIN, PR, T>, T>(
            ),
            delta2: T::zero(),
            resample: None::new(self.project_transform.clone()),
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass
    for BuilderAntimeridianResampleClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type Output = BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    // Switch the builder into one without a resample node,
    fn precision_bypass(&self) -> Self::Output {
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.

        // Copy - Mutate.
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            t360: self.t360,
            theta: self.theta,
            rotate: self.rotate.clone(),
            rotator: self.rotator.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            postclip: self.postclip.clone(),
            alpha: self.alpha,
            lambda: self.lambda,
            phi: self.phi,
            projection_raw: self.projection_raw.clone(),
            k: self.k,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,

            // Mutate section.
            clip: gen_clip_antimeridian::<ResampleNonePCNC<DRAIN, PR, T>, T>(),
            delta2: T::zero(),
            resample: None::new(self.project_transform.clone()),
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderCircleResampleNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    type Output = BuilderCircleResampleNoneNoClip<DRAIN, PR, T>;
    type T = T;

    // Switch the builder into one without a resample node,
    #[inline]
    fn precision_bypass(&self) -> Self::Output {
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.

        // Copy - Mutate.
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            t360: self.t360,
            theta: self.theta,
            rotate: self.rotate.clone(),
            rotator: self.rotator.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            postclip: self.postclip.clone(),
            alpha: self.alpha,
            lambda: self.lambda,
            phi: self.phi,
            projection_raw: self.projection_raw.clone(),
            k: self.k,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,

            // Mutate section.
            clip: gen_clip_circle::<ResampleNoneNoPCNC<DRAIN, PR, T>, T>(
                self.theta.unwrap(),
            ),
            delta2: T::zero(),
            resample: None::new(self.project_transform.clone()),
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderCircleResampleClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    type Output = BuilderCircleResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    // Switch the builder into one without a resample node,
    fn precision_bypass(&self) -> Self::Output {
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.

        let clip = gen_clip_circle::<ResampleNonePCNC<DRAIN, PR, T>, T>(
            self.theta.unwrap(),
        );

        // Copy - Mutate.
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            t360: self.t360,
            theta: self.theta,
            rotate: self.rotate.clone(),
            rotator: self.rotator.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            postclip: self.postclip.clone(),
            alpha: self.alpha,
            lambda: self.lambda,
            phi: self.phi,
            projection_raw: self.projection_raw.clone(),
            k: self.k,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,

            // Mutate section.
            clip,
            delta2: T::zero(),
            resample: None::new(self.project_transform.clone()),
        }
    }
}
