use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleClipC;
use crate::projection::builder::ResampleClipU;
use crate::projection::resampler::resample::Resample;
use crate::projection::transform::generate as generate_str;
use crate::projection::RecenterWithResampling;
use crate::rot::rotate_radians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> RecenterWithResampling
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        NoClipU<DRAIN>,
        PR,
        PV,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter_with_resampling(mut self) -> Self {
        assert!(!self.delta2.is_zero());
        let center = generate_str(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            &self.alpha,
        )
        .transform(&self.projection_raw.transform(&Coordinate {
            x: self.lambda,
            y: self.phi,
        }));
        let transform = generate_str(
            &self.k,
            &(self.x - center.x),
            &(self.y - center.y),
            &self.sx,
            &self.sy,
            &self.alpha,
        );

        let rotate = rotate_radians([self.delta_lambda, self.delta_phi, self.delta_gamma]);
        let project_transform = Compose::new(self.projection_raw.clone(), transform);
        let project_rotate_transform = Compose::new(rotate.clone(), project_transform.clone());
        let rotator = RotatorRadians::new(rotate.clone());
        let resample = Resample::new(project_transform.clone(), self.delta2);

        self.rotate = rotate;
        self.rotator = rotator;
        self.resample = resample;
        self.project_transform = project_transform;
        self.project_rotate_transform = project_rotate_transform;
        self
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> RecenterWithResampling
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        ClipU<DRAIN, T>,
        PR,
        PV,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
        T,
    >
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter_with_resampling(mut self) -> Self {
        assert!(!self.delta2.is_zero());
        let center = generate_str(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            &self.alpha,
        )
        .transform(&self.projection_raw.transform(&Coordinate {
            x: self.lambda,
            y: self.phi,
        }));
        let transform = generate_str(
            &self.k,
            &(self.x - center.x),
            &(self.y - center.y),
            &self.sx,
            &self.sy,
            &self.alpha,
        );

        let rotate = rotate_radians([self.delta_lambda, self.delta_phi, self.delta_gamma]);
        let project_transform = Compose::new(self.projection_raw.clone(), transform);
        let project_rotate_transform = Compose::new(rotate.clone(), project_transform.clone());
        let rotator = RotatorRadians::new(rotate.clone());
        let resample = Resample::new(project_transform.clone(), self.delta2);

        self.rotate = rotate;
        self.rotator = rotator;
        self.resample = resample;
        self.project_transform = project_transform;
        self.project_rotate_transform = project_rotate_transform;
        self
    }
}
