//! # `fit_no_rectangle`.
//!
//! 1) No-op: No rectange to remove.
//! 2) Perform operations.
//! 3) SWAP -  `NoClip` for `Clip`
//!
//! # Elsewhere in `fit_no_rectangle`.
//!
//! 1) Removed Post Clip Rectangle.
//! 2) Perform operations.
//! 3) Restore Post Clip Rectangle
//!

use std::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ClipConnectable;
use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::projection::builder::template::NoPCNC;
use crate::projection::builder::template::NoPCNU;
use crate::projection::Build;
use crate::projection::ScaleSet;
use crate::projection::TranslateSet;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

/// `no_clip` in the sense that input is  `NoClip` (Identity)
/// and so is the output.
fn fit_no_clip<B, CC, CU, FB, PR, RC, RU, T>(
    builder: &mut B,
    mut fit_bounds: FB,
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CC,
            ClipU = CU,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    FB: FnMut([Coord<T>; 2], &mut B),
    CU: Clone + ClipConnectable<Output = CC, SC = RC>,
    CC: Clone + Stream<EP = Bounds<T>, T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let builder = builder
        .scale_set(T::from(150.0_f64).unwrap())
        .translate_set(&Coord {
            x: T::zero(),
            y: T::zero(),
        });
    let bounds_stream = Bounds::<T>::default();
    let mut stream_in = builder.build().stream(&bounds_stream);

    object.to_stream(&mut stream_in);
    let bounds = stream_in.endpoint().result();
    fit_bounds(bounds, builder);
}

pub(super) fn fit_extent_no_clip<B, CC, CU, PR, RC, RU, T>(
    builder: &mut B,
    extent: [[T; 2]; 2],
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CC,
            ClipU = CU,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CU: Clone + ClipConnectable<Output = CC, SC = RC>,
    CC: Clone + Stream<EP = Bounds<T>, T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip(
        builder,
        move |b: [Coord<T>; 2], builder: &mut B| {
            let w = extent[1][0] - extent[0][0];
            let h = extent[1][1] - extent[0][1];
            let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
            let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
            let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

            builder
                .scale_set(one_five_zero * k)
                .translate_set(&Coord { x, y });
        },
        object,
    );
}

pub(super) fn fit_size_no_clip<B, CC, CU, PR, RC, RU, T>(
    builder: &mut B,
    size: [T; 2],
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CC,
            ClipU = CU,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CU: Clone + ClipConnectable<Output = CC, SC = RC>,
    CC: Clone + Stream<EP = Bounds<T>, T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    fit_extent_no_clip(builder, [[T::zero(), T::zero()], size], object);
}

pub(super) fn fit_width_no_clip<B, CC, CU, PR, RC, RU, T>(
    builder: &mut B,
    width: T,
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CC,
            ClipU = CU,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CU: Clone + ClipConnectable<Output = CC, SC = RC>,
    CC: Clone + Stream<EP = Bounds<T>, T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip(
        builder,
        move |b: [Coord<T>; 2], builder: &mut B| {
            let w = width;
            let k = w / (b[1].x - b[0].x);
            let x = (w - k * (b[1].x + b[0].x)) / two;
            let y = -k * b[0].y;

            builder
                .scale_set(one_five_zero * k)
                .translate_set(&Coord { x, y });
        },
        object,
    );
}

pub(super) fn fit_height_no_clip<B, CC, CU, PR, RC, RU, T>(
    builder: &mut B,
    height: T,
    object: &impl Streamable<T = T>,
) where
    PR: Clone + Transform<T = T>,
    B: Build<
            ClipC = CC,
            ClipU = CU,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CU: Clone + ClipConnectable<Output = CC, SC = RC>,
    CC: Clone + Stream<EP = Bounds<T>, T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip(
        builder,
        move |b: [Coord<T>; 2], builder: &mut B| {
            let h = height;
            let k = h / (b[1].y - b[0].y);
            let x = -k * b[0].x;
            let y = (h - k * (b[1].y + b[0].y)) / two;

            builder
                .scale_set(one_five_zero * k)
                .translate_set(&Coord { x, y });
        },
        object,
    );
}
