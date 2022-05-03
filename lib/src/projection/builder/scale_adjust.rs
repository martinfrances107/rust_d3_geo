use crate::projection::builder::BuilderCircleResampleNoneNoClip;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::types::BuilderAntimeridianResampleClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneNoClip;
use crate::projection::builder::types::BuilderCircleResampleClip;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::ScaleAdjust;
use crate::stream::Stream;
use crate::Transform;

impl<DRAIN, PR, T> ScaleAdjust for BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, PR, T> ScaleAdjust for BuilderAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, PR, T> ScaleAdjust for BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}

impl<DRAIN, PR, T> ScaleAdjust for BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}

impl<DRAIN, PR, T> ScaleAdjust for BuilderCircleResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, PR, T> ScaleAdjust for BuilderCircleResampleClip<DRAIN, PR, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, PR, T> ScaleAdjust for BuilderCircleResampleNoneNoClip<DRAIN, PR, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}

// impl<DRAIN, PR, T> ScaleAdjust for BuilderCircleResampleNoneClip<DRAIN, PR, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T>,
//     PR: Clone + Transform<T = T>,
//     T: CoordFloat + FloatConst,
// {
//     type T = T;

//     fn scale(mut self, scale: T) -> Self {
//         self.k = scale;
//         self.recenter_no_resampling()
//     }
// }
