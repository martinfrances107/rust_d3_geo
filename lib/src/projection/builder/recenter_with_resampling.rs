use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::builder::template::ResampleNoPCNU;
use crate::projection::builder::NoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::builder::PCNU;
use crate::projection::resampler::resample::Resample;
use crate::projection::transform::generate as generate_str;
use crate::projection::RecenterWithResampling;
use crate::rot::rotate_radians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::Transform;

use super::template::ResampleNoPCNC;
use super::template::ResamplePCNC;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> RecenterWithResampling
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU,
        PR,
        ResampleNoPCNC<DRAIN, PR, T>,
        ResampleNoPCNU<PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter_with_resampling(&mut self) -> &mut Self {
        assert!(!self.delta2.is_zero());
        let center = generate_str(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            &self.alpha,
        )
        .transform(&self.projection_raw.transform(&Coord {
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

impl<CLIPC, CLIPU, DRAIN, PR, T> RecenterWithResampling
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter_with_resampling(&mut self) -> &mut Self {
        assert!(!self.delta2.is_zero());
        let center = generate_str(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            &self.alpha,
        )
        .transform(&self.projection_raw.transform(&Coord {
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
