use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip as gen_clip_antimeridian;
use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::circle::gen_clip as gen_clip_circle;
use crate::clip::circle::ClipCircleC;
use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::types::BuilderAntimeridianResampleClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleClip;
use crate::projection::builder::types::BuilderCircleResampleNoneClip;
use crate::projection::builder::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::BuilderAntimeridianResampleNoneNoClip;
use crate::projection::builder::BuilderCircleResampleNoClip;
use crate::projection::builder::BuilderCircleResampleNoneNoClip;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Resample;
use crate::projection::ClipExtentSet;

use super::template::PCNU;

// Code Repeated 2^2 times.
// Variantion over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// Varariantion over Resample/None as Resample is rebuilt.
impl<DRAIN, PR, T> ClipExtentSet for BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type Output = BuilderAntimeridianResampleClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coordinate<T>; 2]) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>>,
            p_drain: self.p_drain,
            p_rc: PhantomData::<ResamplePCNC<DRAIN, PR, T>>,
            projection_raw: self.projection_raw.clone(),
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
            rotate: self.rotate.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            rotator: self.rotator.clone(),

            // Mutate section.
            clip: gen_clip_antimeridian::<PCNU<T>, ResamplePCNC<DRAIN, PR, T>, T>(),
            postclip: Rectangle::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
            resample: Resample::new(self.project_transform.clone(), self.delta2),
            x0: Some(extent[0].x),
            y0: Some(extent[0].y),
            x1: Some(extent[1].x),
            y1: Some(extent[1].y),
        }
    }
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type Output = BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coordinate<T>; 2]) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>>,

            p_drain: self.p_drain,
            p_rc: PhantomData::<ResampleNonePCNC<DRAIN, PR, T>>,
            projection_raw: self.projection_raw.clone(),
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
            rotate: self.rotate.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            rotator: self.rotator.clone(),

            // Mutate section.
            clip: gen_clip_antimeridian::<PCNU<T>, ResampleNonePCNC<DRAIN, PR, T>, T>(),
            postclip: Rectangle::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
            resample: None::new(self.project_transform.clone()),
            x0: Some(extent[0].x),
            y0: Some(extent[0].y),
            x1: Some(extent[1].x),
            y1: Some(extent[1].y),
        }
    }
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderCircleResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + FloatConst,
{
    type Output = BuilderCircleResampleClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coordinate<T>; 2]) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipCircleC<ResamplePCNC<DRAIN, PR, T>, T>>,
            p_drain: self.p_drain,
            p_rc: PhantomData::<ResamplePCNC<DRAIN, PR, T>>,
            projection_raw: self.projection_raw.clone(),
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
            rotate: self.rotate.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            rotator: self.rotator.clone(),

            // Mutate section.
            clip: gen_clip_circle::<
                DRAIN,
                PCNU<T>,
                PR,
                ResamplePCNC<DRAIN, PR, T>,
                ResamplePCNU<PR, T>,
                T,
            >(self.theta.unwrap()),
            postclip: Rectangle::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
            resample: Resample::new(self.project_transform.clone(), self.delta2),
            x0: Some(extent[0].x),
            y0: Some(extent[0].y),
            x1: Some(extent[1].x),
            y1: Some(extent[1].y),
        }
    }
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderCircleResampleNoneNoClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + FloatConst,
{
    type T = T;
    type Output = BuilderCircleResampleNoneClip<DRAIN, PR, T>;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coordinate<T>; 2]) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipCircleC<ResampleNonePCNC<DRAIN, PR, T>, T>>,
            p_drain: self.p_drain,
            p_rc: PhantomData::<ResampleNonePCNC<DRAIN, PR, T>>,
            projection_raw: self.projection_raw.clone(),
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
            rotate: self.rotate.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            rotator: self.rotator.clone(),

            // Mutate section.
            clip: gen_clip_circle::<
                DRAIN,
                PCNU<T>,
                PR,
                ResampleNonePCNC<DRAIN, PR, T>,
                ResampleNonePCNU<PR, T>,
                T,
            >(self.theta.unwrap()),
            resample: None::new(self.project_transform.clone()),
            postclip: Rectangle::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
            x0: Some(extent[0].x),
            y0: Some(extent[0].y),
            x1: Some(extent[1].x),
            y1: Some(extent[1].y),
        }
        //TODO javascipt calls reset here.
    }
}
