use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::Clip;
use crate::projection::resampler::none::None;
use crate::projection::PrecisionBypass;

use super::types::BuilderAntimeridianResampleClip;
use super::types::BuilderAntimeridianResampleNoClip;
use super::types::BuilderAntimeridianResampleNoneNoClip;
use super::types::BuilderCircleResampleClip;
use super::types::BuilderCircleResampleNoClip;
use super::types::BuilderCircleResampleNoneClip;
use super::types::BuilderCircleResampleNoneNoClip;

impl<DRAIN, PR, T> PrecisionBypass for BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type Output = BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T>;
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_bypass(self) -> Self::Output {
        let pv = PVAntimeridian::default();
        let interpolator = InterpolateAntimeridian::default();
        let line = LineAntimeridian::default();
        let resample = None::new(self.project_transform.clone());
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.
        let clip = Clip::new(interpolator, line, pv, self.clip.start);

        // Copy - Mutate.
        Self::Output {
            p_lb: self.p_lb,
            p_drain: self.p_drain,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
            theta: self.theta,
            rotate: self.rotate,
            rotator: self.rotator,
            project_transform: self.project_transform,
            project_rotate_transform: self.project_rotate_transform,
            postclip: self.postclip,
            alpha: self.alpha,
            lambda: self.lambda,
            phi: self.phi,
            projection_raw: self.projection_raw,
            k: self.k,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,

            // Mutate section.
            clip,
            delta2: T::zero(),
            resample,
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type Output = BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_bypass(self) -> Self::Output {
        let pv = PVAntimeridian::default();
        let interpolator = InterpolateAntimeridian::default();
        let line = LineAntimeridian::default();
        let resample = None::new(self.project_transform.clone());
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.
        let clip = Clip::new(interpolator, line, pv, self.clip.start);

        // Copy - Mutate.
        Self::Output {
            p_lb: self.p_lb,
            p_drain: self.p_drain,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
            theta: self.theta,
            rotate: self.rotate,
            rotator: self.rotator,
            project_transform: self.project_transform,
            project_rotate_transform: self.project_rotate_transform,
            postclip: self.postclip,
            alpha: self.alpha,
            lambda: self.lambda,
            phi: self.phi,
            projection_raw: self.projection_raw,
            k: self.k,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,

            // Mutate section.
            clip,
            delta2: T::zero(),
            resample,
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderCircleResampleNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: CoordFloat + FloatConst,
{
    type Output = BuilderCircleResampleNoneNoClip<DRAIN, PR, T>;
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_bypass(self) -> Self::Output {
        let radius = self.clip.interpolator.radius;
        let pv = PVCircle::new(radius);
        let interpolator = InterpolateCircle::new(radius);
        let line = LineCircle::default();
        let resample = None::new(self.project_transform.clone());
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.
        let clip = Clip::new(interpolator, line, pv, self.clip.start);

        // Copy - Mutate.
        Self::Output {
            p_lb: self.p_lb,
            p_drain: self.p_drain,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
            theta: self.theta,
            rotate: self.rotate,
            rotator: self.rotator,
            project_transform: self.project_transform,
            project_rotate_transform: self.project_rotate_transform,
            postclip: self.postclip,
            alpha: self.alpha,
            lambda: self.lambda,
            phi: self.phi,
            projection_raw: self.projection_raw,
            k: self.k,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,

            // Mutate section.
            clip,
            delta2: T::zero(),
            resample,
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderCircleResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + FloatConst,
{
    type Output = BuilderCircleResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_bypass(self) -> Self::Output {
        // This is a bodge radius info in embedded in the start variable.
        let pv = PVCircle::new(-self.clip.start.y);
        let interpolator = InterpolateCircle::new(-self.clip.start.y);
        let line = LineCircle::default();
        let resample = None::new(self.project_transform.clone());
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.
        let clip = Clip::new(interpolator, line, pv, self.clip.start);

        // Copy - Mutate.
        Self::Output {
            p_lb: self.p_lb,
            p_drain: self.p_drain,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
            theta: self.theta,
            rotate: self.rotate,
            rotator: self.rotator,
            project_transform: self.project_transform,
            project_rotate_transform: self.project_rotate_transform,
            postclip: self.postclip,
            alpha: self.alpha,
            lambda: self.lambda,
            phi: self.phi,
            projection_raw: self.projection_raw,
            k: self.k,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,

            // Mutate section.
            clip,
            delta2: T::zero(),
            resample,
        }
    }
}
