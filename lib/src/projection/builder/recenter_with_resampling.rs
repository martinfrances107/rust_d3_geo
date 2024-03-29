use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::compose::Compose;
use crate::identity::Identity;
use crate::projection::builder::template::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::resampler::resample::Resample;
use crate::projection::transform::generate as generate_str;
use crate::projection::Recenter;
use crate::rot::rotate_radians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<CLIPU, DRAIN, PR, T> Recenter
    for Builder<
        CLIPU,
        DRAIN,
        Identity<Unconnected>,
        PR,
        ResampleNoPCNU<PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter(&mut self) -> &mut Self {
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

        let rotate = rotate_radians([
            self.delta_lambda,
            self.delta_phi,
            self.delta_gamma,
        ]);
        let project_transform =
            Compose::new(self.projection_raw.clone(), transform);
        let project_rotate_transform =
            Compose::new(rotate.clone(), project_transform.clone());
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

impl<CLIPU, DRAIN, PR, T> Recenter
    for Builder<
        CLIPU,
        DRAIN,
        Rectangle<Unconnected, T>,
        PR,
        ResamplePCNU<PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter(&mut self) -> &mut Self {
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

        let rotate = rotate_radians([
            self.delta_lambda,
            self.delta_phi,
            self.delta_gamma,
        ]);
        let project_transform =
            Compose::new(self.projection_raw.clone(), transform);
        let project_rotate_transform =
            Compose::new(rotate.clone(), project_transform.clone());
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
