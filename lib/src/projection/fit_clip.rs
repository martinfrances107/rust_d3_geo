//! # fit rectangle.
//!
//! 1) Removed Post Clip Rectangle.
//! 2) Perform operations.
//! 3) Restore Post Clip Rectangle
//!
//! # Elsewhere  in `fit_no_rectangle`.
//!
//! 1) No-op: No rectange to remove.
//! 2) Perform operations.
//! 3) SWAP -  implies inserting `PostClip` rectangle.
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
use crate::projection::builder::template::PCNC;
use crate::projection::builder::template::PCNU;
use crate::projection::Build;
use crate::projection::ClipExtentGet;
use crate::projection::ScaleSet;
use crate::projection::TranslateSet;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::ClipExtentClear;
use super::ClipExtentSet;

pub(super) fn fit_clip<B, Bint, CLIPC, CLIPCint, CLIPU, CLIPUint, FB, PR, RC, RCint, RU, RUint, T>(
    builder: &mut B,
    mut fit_bounds: FB,
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CLIPC,
            ClipU = CLIPU,
            Drain = Bounds<T>,
            PCNU = PCNU<T>,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + ClipExtentGet<T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>
        + ClipExtentClear<Output = Bint, T = T>,

    Bint: Build<
            ClipC = CLIPCint,
            ClipU = CLIPUint,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RCint,
            RU = RUint,
            T = T,
        > + ClipExtentSet<Output = B, T = T>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPUint: Clone + ClipConnectable<Output = CLIPCint, SC = RCint>,
    FB: FnMut([Coord<T>; 2], &mut Bint),
    // NB constraints below relate to Bint only not B.
    // They assume no NoClip...
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC>,
    RUint: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RCint>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RCint: Clone + Stream<EP = Bounds<T>, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    let clip = builder.clip_extent();
    let b = builder;
    b.scale_set(T::from(150_f64).unwrap());
    b.translate_set(&Coord {
        x: T::zero(),
        y: T::zero(),
    });
    let mut b_no_clip = b.clip_extent_clear();

    let mut stripped_projector = b_no_clip.build();
    let bounds_stream = Bounds::default();
    let mut stream_in = stripped_projector.stream(&bounds_stream);
    object.to_stream(&mut stream_in);
    let bounds = stream_in.endpoint().result();
    fit_bounds(bounds, &mut b_no_clip);

    b_no_clip.clip_extent_set(&clip);
}

pub(super) fn fit_extent_clip<
    B,
    Bint,
    CLIPC,
    CLIPCint,
    CLIPU,
    CLIPUint,
    PR,
    RC,
    RCint,
    RU,
    RUint,
    T,
>(
    builder: &mut B,
    extent: [Coord<T>; 2],
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CLIPC,
            ClipU = CLIPU,
            Drain = Bounds<T>,
            PCNU = PCNU<T>,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + ClipExtentClear<Output = Bint, T = T>
        + ScaleSet<T = T>
        + ClipExtentGet<T = T>
        + TranslateSet<T = T>,
    Bint: Build<
            ClipC = CLIPCint,
            ClipU = CLIPUint,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RCint,
            RU = RUint,
            T = T,
        > + ClipExtentSet<Output = B, T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ClipConnectable<Output = CLIPC, SC = RC>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPUint: Clone + ClipConnectable<Output = CLIPCint, SC = RCint>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RCint: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC>,
    RUint: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RCint>,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_clip::<B, Bint, CLIPC, CLIPCint, CLIPU, CLIPUint, _, PR, RC, RCint, RU, RUint, T>(
        builder,
        move |b: [Coord<T>; 2], builder: &mut Bint| {
            let w = extent[1].x - extent[0].x;
            let h = extent[1].y - extent[0].y;
            let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
            let x = extent[0].x + (w - k * (b[1].x + b[0].x)) / two;
            let y = extent[0].y + (h - k * (b[1].y + b[0].y)) / two;

            builder
                .scale_set(one_five_zero * k)
                .translate_set(&Coord { x, y });
        },
        object,
    );
}

pub(super) fn fit_size_clip<
    B,
    Bint,
    CLIPC,
    CLIPCint,
    CLIPU,
    CLIPUint,
    PR,
    RC,
    RCint,
    RU,
    RUint,
    T,
>(
    builder: &mut B,
    size: Coord<T>,
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CLIPC,
            ClipU = CLIPU,
            Drain = Bounds<T>,
            PCNU = PCNU<T>,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ClipExtentClear<Output = Bint, T = T>
        + ClipExtentGet<T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,

    Bint: Build<
            ClipC = CLIPCint,
            ClipU = CLIPUint,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RCint,
            RU = RUint,
            T = T,
        > + ClipExtentSet<Output = B, T = T>
        + TranslateSet<T = T>
        + ScaleSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ClipConnectable<Output = CLIPC, SC = RC>,
    CLIPUint: Clone + ClipConnectable<Output = CLIPCint, SC = RCint>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RCint: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC> + Debug,
    RUint: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RCint> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    fit_extent_clip::<B, Bint, CLIPC, CLIPCint, CLIPU, CLIPUint, PR, RC, RCint, RU, RUint, T>(
        builder,
        [
            Coord {
                x: T::zero(),
                y: T::zero(),
            },
            size,
        ],
        object,
    );
}

pub(super) fn fit_width_clip<
    B,
    Bint,
    CLIPC,
    CLIPCint,
    CLIPU,
    CLIPUint,
    PR,
    RC,
    RCint,
    RU,
    RUint,
    T,
>(
    builder: &mut B,
    width: T,
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CLIPC,
            ClipU = CLIPU,
            Drain = Bounds<T>,
            PCNU = PCNU<T>,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ClipExtentGet<T = T>
        + ClipExtentClear<Output = Bint, T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    Bint: Build<
            ClipC = CLIPCint,
            ClipU = CLIPUint,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RCint,
            RU = RUint,
            T = T,
        > + ClipExtentSet<Output = B, T = T>
        + TranslateSet<T = T>
        + ScaleSet<T = T>,
    CLIPU: Clone + ClipConnectable<Output = CLIPC, SC = RC>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPUint: Clone + ClipConnectable<Output = CLIPCint, SC = RCint>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RCint: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC> + Debug,
    RUint: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RCint> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_clip::<B, Bint, CLIPC, CLIPCint, CLIPU, CLIPUint, _, PR, RC, RCint, RU, RUint, T>(
        builder,
        move |b: [Coord<T>; 2], builder: &mut Bint| {
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

pub(super) fn fit_height_clip<
    B,
    Bint,
    CLIPC,
    CLIPCint,
    CLIPU,
    CLIPUint,
    PR,
    RC,
    RCint,
    RU,
    RUint,
    T,
>(
    builder: &mut B,
    height: T,
    object: &impl Streamable<T = T>,
) where
    B: Build<
            ClipC = CLIPC,
            ClipU = CLIPU,
            Drain = Bounds<T>,
            PCNU = PCNU<T>,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        > + Clone
        + ClipExtentGet<T = T>
        + ClipExtentClear<Output = Bint, T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,

    Bint: Build<
            ClipC = CLIPCint,
            ClipU = CLIPUint,
            Drain = Bounds<T>,
            PCNU = NoPCNU,
            PR = PR,
            RC = RCint,
            RU = RUint,
            T = T,
        > + ClipExtentSet<Output = B, T = T>
        + TranslateSet<T = T>
        + ScaleSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ClipConnectable<Output = CLIPC, SC = RC>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPUint: Clone + ClipConnectable<Output = CLIPCint, SC = RCint>,
    PR: Clone + Transform<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RCint: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC> + Debug,
    RUint: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RCint> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_clip::<B, Bint, CLIPC, CLIPCint, CLIPU, CLIPUint, _, PR, RC, RCint, RU, RUint, T>(
        builder,
        move |b: [Coord<T>; 2], builder: &mut Bint| {
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
