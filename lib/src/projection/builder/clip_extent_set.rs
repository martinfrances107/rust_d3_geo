use core::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip as gen_clip_antimeridian;
use crate::clip::circle::gen_clip as gen_clip_circle;
use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::types::BuilderAntimeridianResampleClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleClip;
use crate::projection::builder::types::BuilderCircleResampleNoneClip;
use crate::projection::builder::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::BuilderAntimeridianResampleNoneNoClip;
use crate::projection::builder::BuilderCircleResampleNoClip;
use crate::projection::builder::BuilderCircleResampleNoneNoClip;
use crate::projection::ClipExtentSet;

// Code Repeated 2^2 times.
// Variantion over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// Varariantion over Resample/None as Resample is rebuilt.
impl<DRAIN, PR, T> ClipExtentSet for BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type Output = BuilderAntimeridianResampleClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coord<T>; 2]) -> Self::Output {
        Self::Output {
            p_d: PhantomData::<DRAIN>,
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
            resample: self.resample.clone(),

            // Mutate section.
            clip: gen_clip_antimeridian::<ResamplePCNC<DRAIN, PR, T>, T>(),
            postclip: Rectangle::new(extent),
        }
    }
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type Output = BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coord<T>; 2]) -> Self::Output {
        Self::Output {
            p_d: PhantomData::<DRAIN>,
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
            resample: self.resample.clone(),

            // Mutate section.
            clip: gen_clip_antimeridian::<ResampleNonePCNC<DRAIN, PR, T>, T>(),
            postclip: Rectangle::new(extent),
        }
    }
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderCircleResampleNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    type Output = BuilderCircleResampleClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coord<T>; 2]) -> Self::Output {
        Self::Output {
            p_d: PhantomData::<DRAIN>,
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
            resample: self.resample.clone(),

            // Mutate section.
            clip: gen_clip_circle::<ResamplePCNC<DRAIN, PR, T>, T>(self.theta.unwrap()),
            postclip: Rectangle::new(extent),
        }
    }
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderCircleResampleNoneNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;
    type Output = BuilderCircleResampleNoneClip<DRAIN, PR, T>;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coord<T>; 2]) -> Self::Output {
        Self::Output {
            p_d: PhantomData::<DRAIN>,
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
            resample: self.resample.clone(),

            // Mutate section.
            clip: gen_clip_circle::<ResampleNonePCNC<DRAIN, PR, T>, T>(self.theta.unwrap()),
            postclip: Rectangle::new(extent),
        }
        //TODO javascipt calls reset here.
    }
}
