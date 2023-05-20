//! # `fit_rectangle`.
//!
//! 1) Removed Post Clip Rectangle.
//! 2) Perform operations.
//! 3) Restore Post Clip Rectangle
//!
//! # Elsewhere  in `fit_no_rectangle`.
//!
//! 1) No-op: No rectange to remove.
//! 2) Perform operations.
//! 3) SWAP -  implies inserting `PostClip` Rectangle.
//!

use num_traits::FloatConst;

use geo::CoordFloat;
use geo_types::Coord;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::projection::builder::template::PCNC;
use crate::projection::builder::template::PCNU;
use crate::projection::projector_commom::Projector;
use crate::projection::Build;
use crate::projection::ClipExtentGet;
use crate::projection::ScaleSet;
use crate::projection::TranslateSet;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::ClipExtentAdjust;
use super::Projector as ProjectorTrait;
use super::TranslateGet;

/// `reclip` in the sense that the projection builder
/// internals call reclip()
///
/// reclip ensures that the postclip node will always be
/// set to "Clip" [ A rectangle object, never Identity. ]
///
/// This variant to fit is smaller.
pub(super) fn fit_reclip<B, CLIPC, CLIPU, FB, PR, RC, RU, T>(
    builder: &B,
    mut fit_bounds: FB,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<Projector<Bounds<T>> = Projector<CLIPC, CLIPU, Bounds<T>, PCNU<T>, PR, RU, T>>
        + Clone
        + ClipExtentGet<T = T>
        + ClipExtentAdjust<T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    FB: FnMut([Coord<T>; 2], &B) -> B,
    PR: Transform<T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    let mut b = builder.clone();
    b.scale_set(T::from(150_f64).unwrap());
    b.translate_set(&Coord {
        x: T::zero(),
        y: T::zero(),
    });

    let mut projector = b.build::<Bounds<T>>();
    let bounds_stream = Bounds::default();
    let mut stream_in = projector.stream(&bounds_stream);
    object.to_stream(&mut stream_in);
    let bounds = stream_in.endpoint().result();
    fit_bounds(bounds, &mut b)
}

pub(super) fn fit_extent_reclip<B, CLIPC, CLIPU, PR, RC, RU, T>(
    builder: &B,
    extent: [Coord<T>; 2],
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<Projector<Bounds<T>> = Projector<CLIPC, CLIPU, Bounds<T>, PCNU<T>, PR, RU, T>>
        + Clone
        + ClipExtentGet<T = T>
        + ClipExtentAdjust<T = T>
        + ScaleSet<T = T>
        + TranslateGet<T = T>
        + TranslateSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    PR: Transform<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC>,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_reclip::<_, CLIPC, _, _, _, _, _, _>(
        builder,
        |b: [Coord<T>; 2], builder: &B| -> B {
            let w = extent[1].x - extent[0].x;
            let h = extent[1].y - extent[0].y;
            let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
            let x = extent[0].x + (w - k * (b[1].x + b[0].x)) / two;
            let y = extent[0].y + (h - k * (b[1].y + b[0].y)) / two;
            let mut out: B = builder.clone();
            out.scale_set(one_five_zero * k);
            out.translate_set(&Coord { x, y });
            out
        },
        object,
    )
}

pub(super) fn fit_size_reclip<B, CLIPC, CLIPU, PR, RC, RU, T>(
    builder: &B,
    size: Coord<T>,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<Projector<Bounds<T>> = Projector<CLIPC, CLIPU, Bounds<T>, PCNU<T>, PR, RU, T>>
        + ClipExtentAdjust<T = T>
        + ClipExtentGet<T = T>
        + Clone
        + ScaleSet<T = T>
        + TranslateGet<T = T>
        + TranslateSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    PR: Transform<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC>,
    T: 'static + CoordFloat + FloatConst,
{
    fit_extent_reclip::<_, CLIPC, _, _, _, _, _>(
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

pub(super) fn fit_width_reclip<B, CLIPC, CLIPU, PR, RC, RU, T>(
    builder: &B,
    width: T,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<Projector<Bounds<T>> = Projector<CLIPC, CLIPU, Bounds<T>, PCNU<T>, PR, RU, T>>
        + Clone
        + ClipExtentGet<T = T>
        + ClipExtentAdjust<T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    PR: Transform<T = T>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC>,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_reclip::<_, CLIPC, _, _, _, _, _, _>(
        builder,
        |b: [Coord<T>; 2], builder: &B| -> B {
            let w = width;
            let k = w / (b[1].x - b[0].x);
            let x = (w - k * (b[1].x + b[0].x)) / two;
            let y = -k * b[0].y;

            let mut out = builder.clone();
            out.scale_set(one_five_zero * k);
            out.translate_set(&Coord { x, y });
            out
        },
        object,
    )
}

pub(super) fn fit_height_reclip<B, CLIPC, CLIPU, PR, RC, RU, T>(
    builder: &B,
    height: T,
    object: &impl Streamable<T = T>,
) -> B
where
    PR: Clone + Transform<T = T>,
    B: Build<Projector<Bounds<T>> = Projector<CLIPC, CLIPU, Bounds<T>, PCNU<T>, PR, RU, T>>
        + Clone
        + ClipExtentGet<T = T>
        + ClipExtentAdjust<T = T>
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<PCNC<Bounds<T>, T>> = RC>,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_reclip::<_, CLIPC, _, _, _, _, _, _>(
        builder,
        |b: [Coord<T>; 2], builder: &B| -> B {
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
