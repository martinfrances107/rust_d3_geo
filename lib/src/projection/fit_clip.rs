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

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::projection::builder::template::NoPCNC;
use crate::projection::builder::template::NoPCNU;
use crate::projection::builder::template::PCNC;
use crate::projection::builder::template::PCNU;
use crate::projection::Build;
use crate::projection::ClipExtentGet;
use crate::projection::Projector as ProjectorTrait;
use crate::projection::ScaleSet;
use crate::projection::TranslateSet;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::projector_commom::Projector;
use super::projector_commom::Source;
use super::ClipExtentClear;
use super::ClipExtentSet;
use super::TranslateGet;

pub(super) fn fit_clip<B, Bint, CLIPC, CLIPCint, CLIPU, CLIPUint, FB, PR, RC, RCint, RU, RUint, T>(
    builder: &B,
    mut fit_bounds: FB,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<
                CLIPU,
                Bounds<T>,
                PCNU<T>,
                PR,
                RU,
                Source<CLIPC, T>,
                T,
            >,
        > + Clone
        + ClipExtentGet<T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>
        + ClipExtentClear<Output = Bint, T = T>,

    Bint: Build<
            Projector<Bounds<T>> = Projector<
                CLIPUint,
                Bounds<T>,
                NoPCNU,
                PR,
                RUint,
                Source<CLIPCint, T>,
                T,
            >,
        > + ClipExtentSet<Output = B, T = T>,
    CLIPC: Clone,
    CLIPU: Clone,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPUint: Clone + ConnectableClip<Output = CLIPCint, SC = RCint>,
    FB: FnMut([Coord<T>; 2], &Bint) -> Bint,
    PR: Transform<T = T>,
    // NB constraints below relate to Bint only not B.
    // They assume no NoClip...
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC>,
    RUint: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RCint>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RCint: Clone + Stream<EP = Bounds<T>, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    let clip = builder.clip_extent();
    let mut b = builder.clone();
    b.scale_set(T::from(150_f64).unwrap());
    b.translate_set(&Coord {
        x: T::zero(),
        y: T::zero(),
    });
    let mut b_no_clip = b.clip_extent_clear();

    let mut stripped_projector = b_no_clip.build::<Bounds<T>>();
    let bounds_stream = Bounds::<T>::default();
    let mut stream_in = stripped_projector.stream(&bounds_stream);
    object.to_stream(&mut stream_in);
    let bounds = stream_in.endpoint().result();
    let fb = fit_bounds(bounds, &mut b_no_clip);

    fb.clip_extent_set(&clip)
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
    builder: &B,
    extent: [Coord<T>; 2],
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<
                CLIPU,
                Bounds<T>,
                PCNU<T>,
                PR,
                RU,
                Source<CLIPC, T>,
                T,
            >,
        > + Clone
        + ClipExtentClear<Output = Bint, T = T>
        + ScaleSet<T = T>
        + ClipExtentGet<T = T>
        + TranslateGet<T = T>
        + TranslateSet<T = T>,
    Bint: Build<
            Projector<Bounds<T>> = Projector<
                CLIPUint,
                Bounds<T>,
                NoPCNU,
                PR,
                RUint,
                Source<CLIPCint, T>,
                T,
            >,
        > + Clone
        + ClipExtentSet<Output = B, T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPUint: Clone + ConnectableClip<Output = CLIPCint, SC = RCint>,
    PR: Transform<T = T>,
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
        |b: [Coord<T>; 2], builder: &Bint| -> Bint {
            let w = extent[1].x - extent[0].x;
            let h = extent[1].y - extent[0].y;
            let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
            let x = extent[0].x + (w - k * (b[1].x + b[0].x)) / two;
            let y = extent[0].y + (h - k * (b[1].y + b[0].y)) / two;

            let mut out = builder.clone();
            out.scale_set(one_five_zero * k)
                .translate_set(&Coord { x, y });
            out
        },
        object,
    )
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
    builder: &B,
    size: Coord<T>,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<
                CLIPU,
                Bounds<T>,
                PCNU<T>,
                PR,
                RU,
                Source<CLIPC, T>,
                T,
            >,
        > + Clone
        + ClipExtentClear<Output = Bint, T = T>
        + ClipExtentGet<T = T>
        + ScaleSet<T = T>
        + TranslateGet<T = T>
        + TranslateSet<T = T>,
    Bint: Build<
            Projector<Bounds<T>> = Projector<
                CLIPUint,
                Bounds<T>,
                NoPCNU,
                PR,
                RUint,
                Source<CLIPCint, T>,
                T,
            >,
        > + Clone
        + ClipExtentSet<Output = B, T = T>
        + TranslateSet<T = T>
        + ScaleSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPUint: Clone + ConnectableClip<Output = CLIPCint, SC = RCint>,
    PR: Transform<T = T>,
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
    )
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
    builder: &B,
    width: T,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<
                CLIPU,
                Bounds<T>,
                PCNU<T>,
                PR,
                RU,
                Source<CLIPC, T>,
                T,
            >,
        > + Clone
        + ClipExtentGet<T = T>
        + ClipExtentClear<Output = Bint, T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    Bint: Build<
            Projector<Bounds<T>> = Projector<
                CLIPUint,
                Bounds<T>,
                NoPCNU,
                PR,
                RUint,
                Source<CLIPCint, T>,
                T,
            >,
        > + Clone
        + ClipExtentSet<Output = B, T = T>
        + TranslateSet<T = T>
        + ScaleSet<T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPUint: Clone + ConnectableClip<Output = CLIPCint, SC = RCint>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Transform<T = T>,
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
        |b: [Coord<T>; 2], builder: &Bint| -> Bint {
            let w = width;
            let k = w / (b[1].x - b[0].x);
            let x = (w - k * (b[1].x + b[0].x)) / two;
            let y = -k * b[0].y;

            let mut out: Bint = builder.clone();
            out.scale_set(one_five_zero * k)
                .translate_set(&Coord { x, y });
            out
        },
        object,
    )
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
    builder: &B,
    height: T,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<
                CLIPU,
                Bounds<T>,
                PCNU<T>,
                PR,
                RU,
                Source<CLIPC, T>,
                T,
            >,
        > + Clone
        + ClipExtentGet<T = T>
        + ClipExtentClear<Output = Bint, T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,

    Bint: Build<
            Projector<Bounds<T>> = Projector<
                CLIPUint,
                Bounds<T>,
                NoPCNU,
                PR,
                RUint,
                Source<CLIPCint, T>,
                T,
            >,
        > + Clone
        + ClipExtentSet<Output = B, T = T>
        + TranslateSet<T = T>
        + ScaleSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPCint: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPUint: Clone + ConnectableClip<Output = CLIPCint, SC = RCint>,
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
        |b: [Coord<T>; 2], builder: &Bint| -> Bint {
            let h = height;
            let k = h / (b[1].y - b[0].y);
            let x = -k * b[0].x;
            let y = (h - k * (b[1].y + b[0].y)) / two;

            let mut out = builder.clone();
            out.scale_set(one_five_zero * k);
            out.translate_set(&Coord { x, y });
            out
        },
        object,
    )
}
