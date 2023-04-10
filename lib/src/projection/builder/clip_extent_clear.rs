use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip;
use crate::clip::antimeridian::ClipAntimeridianC;
use crate::identity::Identity;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Resample;
use crate::projection::ClipExtentClear;

use super::template::ResampleNoPCNC;
use super::template::ResampleNoneNoPCNC;
use super::types::BuilderAntimeridianResampleClip;
use super::types::BuilderAntimeridianResampleNoClip;
use super::types::BuilderAntimeridianResampleNoneClip;
use super::types::BuilderAntimeridianResampleNoneNoClip;

// TODO Need 4 varyations here.
// Vary by Antimeridian/Circle
// Vary by Resample/None
impl<DRAIN, PR, T> ClipExtentClear for BuilderAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type T = T;
    type Output = BuilderAntimeridianResampleNoClip<DRAIN, PR, T>;

    #[inline]
    fn clip_extent_clear(&self) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipAntimeridianC<ResampleNoPCNC<DRAIN, PR, T>, T>>,
            p_drain: self.p_drain,
            projection_raw: self.projection_raw.clone(),
            phi: self.phi,
            lambda: self.lambda,
            alpha: self.alpha,
            k: self.k,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            t360: self.t360,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            delta2: self.delta2,
            theta: self.theta,
            rotate: self.rotate.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            rotator: self.rotator.clone(),

            // Mutate section.
            postclip: Identity::default(),
            clip: gen_clip::<ResampleNoPCNC<DRAIN, PR, T>, T>(),
            resample: Resample::new(self.project_transform.clone(), self.delta2),
        }
    }
}

impl<DRAIN, PR, T> ClipExtentClear for BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type T = T;
    type Output = BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T>;

    #[inline]
    fn clip_extent_clear(&self) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipAntimeridianC<ResampleNoneNoPCNC<DRAIN, PR, T>, T>>,
            p_drain: self.p_drain,
            projection_raw: self.projection_raw.clone(),
            phi: self.phi,
            lambda: self.lambda,
            alpha: self.alpha,
            k: self.k,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            t360: self.t360,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            delta2: self.delta2,
            theta: self.theta,
            rotate: self.rotate.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            rotator: self.rotator.clone(),

            // Mutate section.
            clip: gen_clip::<_, _>(),
            postclip: Identity::default(),
            resample: None::new(self.project_transform.clone()),
        }
    }
}
