use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::builder::NoPCNU;
use crate::projection::builder::ResampleNoneNoPCNU;
use crate::projection::builder::ResampleNonePCNU;
use crate::projection::builder::PCNU;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::transform::generate as generate_str;
use crate::projection::RecenterNoResampling;
use crate::rot::rotate_radians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::Transform;

use super::template::ResampleNoneNoPCNC;
use super::template::ResampleNonePCNC;
use super::Builder;

#[allow(clippy::similar_names)]
impl<CLIPC, CLIPU, DRAIN, PR, T> RecenterNoResampling
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU,
        PR,
        ResampleNoneNoPCNC<DRAIN, PR, T>,
        ResampleNoneNoPCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter_no_resampling(&mut self) -> &mut Self {
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
        let resample = ResampleNone::new(project_transform.clone());

        self.rotate = rotate;
        self.rotator = rotator;
        self.resample = resample;
        self.project_transform = project_transform;
        self.project_rotate_transform = project_rotate_transform;
        self
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> RecenterNoResampling
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResampleNonePCNC<DRAIN, PR, T>,
        ResampleNonePCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter_no_resampling(&mut self) -> &mut Self {
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

        let resample = ResampleNone::new(project_transform.clone());

        self.rotate = rotate;
        self.rotator = rotator;
        self.resample = resample;
        self.project_transform = project_transform;
        self.project_rotate_transform = project_rotate_transform;
        self
    }
}
