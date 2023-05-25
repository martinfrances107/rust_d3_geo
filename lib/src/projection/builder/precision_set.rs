use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::Clipper;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::template::ResampleNonePCNC;
use super::template::ResamplePCNC;
use super::Builder;

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
    for Builder<
        ClipAntimeridianU<None<PR, Connected<PCNC>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        None<PR, Unconnected, T>,
        T,
    >
where
    PCNU: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type Output = Builder<
        ClipAntimeridianU<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, Unconnected, T>,
        T,
    >;
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_set(&self, delta: &T) -> Self::Output {
        let pv = PVAntimeridian::default();
        let interpolator = InterpolateAntimeridian::default();
        let line = LineAntimeridian::default();
        let delta2 = *delta * *delta;
        let resample = Resample::new(self.project_transform.clone(), delta2);
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.
        let clip = Clipper::new(interpolator, line, pv, self.clip.start);

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
            clip,

            // Mutate section.
            delta2,
            resample,
        }
    }
}

impl<DRAIN, PR, PCNU, T> PrecisionSet
    for Builder<
        ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
        DRAIN,
        PCNU,
        PR,
        None<PR, Unconnected, T>,
        T,
    >
where
    PCNU: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type Output = Builder<
        ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, Unconnected, T>,
        T,
    >;
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_set(&self, delta: &T) -> Self::Output {
        let radius = self.clip.interpolator.radius;
        let pv = PVCircle::new(radius);
        let interpolator = InterpolateCircle::new(radius);
        let line = LineCircle::default();
        let delta2 = *delta * *delta;
        let resample = Resample::new(self.project_transform.clone(), delta2);
        // Architecture Discussion:
        // CLIP is generic over <.. RC, RU,..>,
        // So a change in the resample type causes rebuilding of clip.
        let clip = Clipper::new(interpolator, line, pv, self.clip.start);

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
            clip,

            // Mutate section.
            delta2,
            resample,
        }
    }
}
