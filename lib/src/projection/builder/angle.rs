use crate::stream::Stream;
use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::Buffer;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::AngleSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::template::ClipC;
use super::template::ClipU;
use super::template::NoClipC;
use super::template::NoClipU;
use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> AngleSet
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        NoClipC<DRAIN, T>,
        NoClipU<DRAIN, T>,
        PR,
        PV,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn angle(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> AngleSet
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        ClipC<DRAIN, T>,
        ClipU<DRAIN, T>,
        PR,
        PV,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn angle(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<DRAIN, PR, T> AngleSet
    for Builder<
        DRAIN,
        InterpolateAntimeridian<T>,
        LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
        LineAntimeridian<
            DRAIN,
            ResampleNoneClipC<DRAIN, PR, T>,
            Connected<ResampleNoneClipC<DRAIN, PR, T>>,
            T,
        >,
        LineAntimeridian<DRAIN, ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
        ClipC<DRAIN, T>,
        ClipU<DRAIN, T>,
        PR,
        PVAntimeridian<T>,
        ResampleNoneClipC<DRAIN, PR, T>,
        ResampleNoneClipU<DRAIN, PR, T>,
        T,
    >
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    fn angle(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_no_resampling()
    }
}
