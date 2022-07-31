//! # fit_no_rectangle.
//!
//! 1) No-op: No rectange to remove.
//! 2) Perform operations.
//! 3) SWAP -  NoClip for Clip
//!
//! # Elsewhere in fit_no_rectangle.
//!
//! 1) Removed Post Clip Rectangle.
//! 2) Perform operations.
//! 3) Restore Post Clip Rectangle
//!

use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::Build;
use crate::projection::ScaleSet;
use crate::projection::TranslateSet;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

/// no_clip in the sense tha input is  NoClip (Identity)
/// and so is the output.
fn fit_no_clip<B, I, LB, LC, LU, PR, PV, RC, RU, T>(
    builder: B,
    fit_bounds: Box<dyn Fn([Coordinate<T>; 2], B) -> B>,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Drain = Bounds<T>,
            I = I,
            LB = LB,
            LC = LC,
            LU = LU,
            PCNU = NoClipU<Bounds<T>>,
            PR = PR,
            PV = PV,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    I: Clone + Interpolator<T = T>,
    LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
    PR: Clone + Transform<T = T>,
    PV: Clone + PointVisible<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output = RC, SC = NoClipC<Bounds<T>>> + Debug,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let builder = builder
        .scale_set(T::from(150.0_f64).unwrap())
        .translate_set(&Coordinate {
            x: T::zero(),
            y: T::zero(),
        });
    let bounds_stream = Bounds::<T>::default();
    let mut stream_in = builder.build().stream(&bounds_stream);

    object.to_stream(&mut stream_in);
    let bounds = stream_in.endpoint().result();
    fit_bounds(bounds, builder)
}

pub(super) fn fit_extent_no_clip<B, I, LB, LC, LU, PR, PV, RC, RU, T>(
    builder: B,
    extent: [[T; 2]; 2],
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Drain = Bounds<T>,
            I = I,
            LB = LB,
            LC = LC,
            LU = LU,
            PCNU = NoClipU<Bounds<T>>,
            PR = PR,
            PV = PV,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    PR: Clone + Transform<T = T>,
    I: Clone + Interpolator<T = T>,
    LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
    PV: Clone + PointVisible<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output = RC, SC = NoClipC<Bounds<T>>> + Debug,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip(
        builder,
        Box::new(move |b: [Coordinate<T>; 2], builder: B| {
            let w = extent[1][0] - extent[0][0];
            let h = extent[1][1] - extent[0][1];
            let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
            let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
            let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

            builder
                .scale_set(one_five_zero * k)
                .translate_set(&Coordinate { x, y })
        }),
        object,
    )
}

pub(super) fn fit_size_no_clip<B, I, LB, LC, LU, PR, PV, RC, RU, T>(
    builder: B,
    size: [T; 2],
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Drain = Bounds<T>,
            I = I,
            LB = LB,
            LC = LC,
            LU = LU,
            PCNU = NoClipU<Bounds<T>>,
            PR = PR,
            PV = PV,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    I: Clone + Interpolator<T = T>,
    LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
    PV: Clone + PointVisible<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output = RC, SC = NoClipC<Bounds<T>>> + Debug,
    PR: Clone + Transform<T = T>,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    fit_extent_no_clip(builder, [[T::zero(), T::zero()], size], object)
}

pub(super) fn fit_width_no_clip<B, I, LB, LC, LU, PR, PV, RC, RU, T>(
    builder: B,
    width: T,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Drain = Bounds<T>,
            I = I,
            LB = LB,
            LC = LC,
            LU = LU,
            PCNU = NoClipU<Bounds<T>>,
            PR = PR,
            PV = PV,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    I: Clone + Interpolator<T = T>,
    LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
    PV: Clone + PointVisible<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output = RC, SC = NoClipC<Bounds<T>>> + Debug,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip(
        builder,
        Box::new(move |b: [Coordinate<T>; 2], builder: B| {
            let w = width;
            let k = w / (b[1].x - b[0].x);
            let x = (w - k * (b[1].x + b[0].x)) / two;
            let y = -k * b[0].y;

            builder
                .scale_set(one_five_zero * k)
                .translate_set(&Coordinate { x, y })
        }),
        object,
    )
}

pub(super) fn fit_height_no_clip<B, I, LB, LC, LU, PR, PV, RC, RU, T>(
    builder: B,
    height: T,
    object: &impl Streamable<T = T>,
) -> B
where
    PR: Clone + Transform<T = T>,
    B: Build<
            Drain = Bounds<T>,
            I = I,
            LB = LB,
            LC = LC,
            LU = LU,
            PCNU = NoClipU<Bounds<T>>,
            PR = PR,
            PV = PV,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    I: Clone + Interpolator<T = T>,
    LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
    PV: Clone + PointVisible<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output = RC, SC = NoClipC<Bounds<T>>> + Debug,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip(
        builder,
        Box::new(move |b: [Coordinate<T>; 2], builder: B| {
            let h = height;
            let k = h / (b[1].y - b[0].y);
            let x = -k * b[0].x;
            let y = (h - k * (b[1].y + b[0].y)) / two;

            builder
                .scale_set(one_five_zero * k)
                .translate_set(&Coordinate { x, y })
        }),
        object,
    )
}
