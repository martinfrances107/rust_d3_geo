use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::identity::Identity;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Resample;
use crate::projection::ClipExtentClear;

use super::template::NoClipC;
use super::template::NoClipU;
use super::template::ResampleNoClipC;
use super::template::ResampleNoneNoClipC;
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
    fn clip_extent_clear(self) -> Self::Output {
        let clip = gen_clip_antimeridian::<NoClipU<DRAIN>, ResampleNoClipC<DRAIN, PR, T>, T>();
        let postclip: Identity<DRAIN, _> = Identity::default();
        let resample: Resample<_, NoClipC<DRAIN>, _, _> =
            Resample::new(self.project_transform.clone(), self.delta2);
        Self::Output {
            p_lb: self.p_lb,
            p_drain: self.p_drain,
            projection_raw: self.projection_raw,
            phi: self.phi,
            lambda: self.lambda,
            alpha: self.alpha,
            k: self.k,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            delta2: self.delta2,
            theta: self.theta,
            rotate: self.rotate,
            project_transform: self.project_transform,
            project_rotate_transform: self.project_rotate_transform,
            rotator: self.rotator,

            // Mutate stage
            postclip,
            clip,
            resample,
            x0: None,
            y0: None,
            x1: None,
            y1: None,
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
    fn clip_extent_clear(self) -> Self::Output {
        let clip = gen_clip_antimeridian::<NoClipU<DRAIN>, ResampleNoneNoClipC<DRAIN, PR, T>, T>();
        let postclip: Identity<DRAIN, _> = Identity::default();
        let resample = None::new(self.project_transform.clone());
        Self::Output {
            p_lb: self.p_lb,
            p_drain: self.p_drain,
            projection_raw: self.projection_raw,
            phi: self.phi,
            lambda: self.lambda,
            alpha: self.alpha,
            k: self.k,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            delta2: self.delta2,
            theta: self.theta,
            rotate: self.rotate,
            project_transform: self.project_transform,
            project_rotate_transform: self.project_rotate_transform,
            rotator: self.rotator,

            // Mutate stage
            postclip,
            clip,
            resample,
            x0: None,
            y0: None,
            x1: None,
            y1: None,
        }
    }
}
