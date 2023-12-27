use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::compose::Compose;
use crate::identity::Identity;
use crate::projection::builder::ResampleNoneNoPCNU;
use crate::projection::builder::ResampleNonePCNU;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::transform::generate as generate_str;
use crate::projection::Recenter;
use crate::rot::rotate_radians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

#[allow(clippy::similar_names)]
impl<CLIPU, DRAIN, PR, T> Recenter
    for Builder<
        CLIPU,
        DRAIN,
        Identity<Unconnected>,
        PR,
        ResampleNoneNoPCNU<PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter(&mut self) -> &mut Self {
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
        let resample = ResampleNone::new(project_transform.clone());

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
        ResampleNonePCNU<PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    fn recenter(&mut self) -> &mut Self {
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

        let resample = ResampleNone::new(project_transform.clone());

        self.rotate = rotate;
        self.rotator = rotator;
        self.resample = resample;
        self.project_transform = project_transform;
        self.project_rotate_transform = project_rotate_transform;
        self
    }
}
