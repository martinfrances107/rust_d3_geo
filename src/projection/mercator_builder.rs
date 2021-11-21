use std::fmt::Debug;
use std::fmt::Display;

use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip_factory_antimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::PointVisible;
use crate::rotation::rotate_radians;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder as ProjectionBuilder;
use super::stream_node_factory::StreamNodeFactory;
use super::stream_transform_radians::StreamTransformRadians;
use super::Angle;
use super::Bounds;
use super::Center;
use super::ClipExtent;
use super::DataObject;
use super::Fit;
use super::Precision;
use super::Projection;
use super::Raw as ProjectionRaw;
use super::Reflect;
use super::Scale;
use super::TransformExtent;
use super::Translate;
use crate::projection::Rotate;

/// A wrapper for Projection\Builder which overrides the traits - scale translate and center.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PV: PointVisible<T = T>,
    PR: ProjectionRaw<T>, // TODO limit this to only certain types of PR
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    pr: PR,
    base: ProjectionBuilder<DRAIN, PR, PV, T>,
    x0: Option<T>,
    y0: Option<T>,
    x1: Option<T>,
    y1: Option<T>, // post-clip extent
}

impl<DRAIN, PR, T> MercatorBuilder<DRAIN, PR, PVAntimeridian<T>, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PR: ProjectionRaw<T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    /// Wrap a default projector and provides mercator specific overrides.
    pub fn new(pr: PR) -> Self {
        let base: ProjectionBuilder<DRAIN, PR, PVAntimeridian<T>, T> =
            ProjectionBuilder::new(gen_clip_factory_antimeridian(), pr.clone());
        Self {
            pr,
            base,
            x0: None,
            y0: None,
            x1: None,
            y1: None,
        }
    }

    /// Using the currently programmed state output a new projection.
    #[inline]
    pub fn build(&self) -> Projection<DRAIN, PR, PVAntimeridian<T>, T> {
        Projection {
            postclip_factory: self.base.postclip_factory.clone(),
            preclip_factory: self.base.preclip_factory.clone(),
            resample_factory: self.base.resample_factory.clone(),

            rotate_transform: self.base.project_rotate_transform.clone(),
            rotate_transform_factory: self.base.rotate_transform_factory.clone(),
            rotate_factory: self.base.rotate_factory.clone(),
            transform_radians_factory: StreamNodeFactory::new(StreamTransformRadians {}),
        }
    }
}

impl<DRAIN, PR, PV, T> MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PR: TransformExtent<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    fn reclip(mut self) -> Self {
        let k = T::PI() * self.get_scale();

        let rotate_raw = self.base.get_rotate();
        let t = rotate_radians(rotate_raw).invert(&Coordinate {
            x: T::zero(),
            y: T::zero(),
        });
        let t = self.base.build().transform(&t);
        let ce = match (self.x0, self.y0, self.x1, self.y1) {
            (Some(x0), Some(y0), Some(x1), Some(y1)) => {
                // MercatorRaw and MercatorTransverseRaw supply different
                // transforms
                self.pr.clone().transform_extent(k, t, x0, y0, x1, y1)
            }
            _ => [
                Coordinate {
                    x: t.x - k,
                    y: t.y - k,
                },
                Coordinate {
                    x: t.x + k,
                    y: t.y + k,
                },
            ],
        };

        self.base = self.base.clip_extent(&ce);
        self
    }
}

impl<DRAIN, PR, PV, T> Center for MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PR: TransformExtent<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_center(&self) -> Coordinate<T> {
        self.base.get_center()
    }

    fn center(mut self, center: &Coordinate<T>) -> Self {
        self.base = self.base.center(center);
        self.reclip()
    }
}

impl<DRAIN, PR, PV, T> Scale for MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PR: TransformExtent<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_scale(&self) -> T {
        self.base.get_scale()
    }

    fn scale(mut self, scale: T) -> Self {
        self.base = self.base.scale(scale);
        self.reclip()
    }
}

impl<DRAIN, PR, PV, T> Translate for MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PR: TransformExtent<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        self.base.get_translate()
    }

    fn translate(mut self, t: &Coordinate<T>) -> Self {
        self.base = self.base.translate(t);
        self.reclip()
    }
}

impl<DRAIN, PR, PV, T> Precision for MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;
    #[inline]
    fn get_precision(&self) -> T {
        self.base.get_precision()
    }

    fn precision(mut self, delta: &T) -> Self {
        self.base = self.base.precision(delta);
        self
    }
}

impl<PR, PV, T> Fit for MercatorBuilder<Bounds<T>, PR, PV, T>
where
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(mut self, extent: [[T; 2]; 2], object: &DataObject<Self::T>) -> Self
    where
        Self::T: AsPrimitive<T> + CoordFloat,
    {
        self.base = self.base.fit_extent(extent, object);
        self
    }

    #[inline]
    fn fit_size(mut self, size: [T; 2], object: &DataObject<T>) -> Self
    where
        Self::T: AsPrimitive<T> + CoordFloat,
    {
        self.base = self.base.fit_size(size, object);
        self
    }
}

impl<DRAIN, PR, PV, T> ClipExtent for MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PR: TransformExtent<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    /// Returns a bounding box.
    fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]> {
        match (self.x0, self.y0, self.x1, self.y1) {
            (Some(x0), Some(y0), Some(x1), Some(y1)) => {
                Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
            }
            _ => None,
        }
    }

    /// clears the bounding box.
    fn clip_extent_clear(mut self) -> Self {
        self.x0 = None;
        self.y0 = None;
        self.x1 = None;
        self.y1 = None;
        self
    }

    /// Sets the bounding box.
    fn clip_extent(mut self, extent: &[Coordinate<Self::T>; 2]) -> Self {
        self.x0 = Some(extent[0].x);
        self.y0 = Some(extent[0].y);
        self.x1 = Some(extent[1].x);
        self.y1 = Some(extent[1].y);
        self.reclip()
    }
}

impl<DRAIN, PR, PV, T> Angle for MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Default + Stream<EP = DRAIN, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_angle(&self) -> T {
        self.base.get_angle()
    }

    /// Sets the rotation angles as measured in degrees.
    fn angle(mut self, angle: T) -> Self {
        self.base = self.base.angle(angle);
        self
    }
}

impl<DRAIN, PR, PV, T> Rotate for MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_rotate(&self) -> [T; 3] {
        self.base.get_rotate()
    }

    /// Sets the rotation angles as measured in degrees.
    fn rotate(mut self, angles: &[T; 3]) -> Self {
        self.base = self.base.rotate(angles);
        self
    }
}

impl<DRAIN, PR, PV, T> Reflect for MercatorBuilder<DRAIN, PR, PV, T>
where
    DRAIN: Default + Stream<EP = DRAIN, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static
        + AbsDiffEq<Epsilon = T>
        + std::ops::AddAssign
        + AsPrimitive<T>
        + CoordFloat
        + Display
        + FloatConst,
{
    type T = T;

    /// Is the projection builder set to invert the x-coordinate.
    #[inline]
    fn get_reflect_x(&self) -> bool {
        self.base.get_reflect_x()
    }

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x(mut self, reflect: bool) -> Self {
        self.base = self.base.reflect_x(reflect);
        self
    }

    /// Is the projection builder set to invert the y-coordinate.
    #[inline]
    fn get_reflect_y(&self) -> bool {
        self.base.get_reflect_y()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y(mut self, reflect: bool) -> Self {
        self.base = self.base.reflect_y(reflect);
        self
    }
}
