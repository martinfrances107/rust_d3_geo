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

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::projection::builder::template::NoPCNC;
use crate::projection::builder::template::NoPCNU;
use crate::projection::projector_commom::Projector;
use crate::projection::Build;
use crate::projection::ScaleSet;
use crate::projection::TranslateSet;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::projector_commom::Source;
use super::Projector as ProjectorTrait;

/// `no_clip` in the sense that input is  `NoClip` (Identity)
/// and so is the output.
fn fit_no_clip<B, CLIPC, CLIPU, FB, PR, RC, RU, T>(
    builder: &B,
    mut fit_bounds: FB,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<CLIPU, Bounds<T>, NoPCNU, PR, RU, Source<CLIPC, T>, T>,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    FB: FnMut([Coord<T>; 2], &B) -> B,
    CLIPU: ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Transform<T = T>,
    RC: Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let mut builder = builder.clone();
    builder.scale_set(T::from(150.0_f64).unwrap());
    builder.translate_set(&Coord {
        x: T::zero(),
        y: T::zero(),
    });
    let bounds_stream = Bounds::<T>::default();
    let mut stream_in = builder.build::<Bounds<T>>().stream(&bounds_stream);

    object.to_stream(&mut stream_in);
    let bounds = stream_in.endpoint().result();
    fit_bounds(bounds, &builder)
}

pub(super) fn fit_extent_no_clip<B, CLIPC, CLIPU, PR, RC, RU, T>(
    builder: &B,
    extent: [Coord<T>; 2],
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<CLIPU, Bounds<T>, NoPCNU, PR, RU, Source<CLIPC, T>, T>,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Transform<T = T>,
    RC: Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip::<_, CLIPC, _, _, _, _, _, _>(
        builder,
        |b: [Coord<T>; 2], builder: &B| -> B {
            let w = extent[1].x - extent[0].x;
            let h = extent[1].y - extent[0].y;
            let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
            let x = extent[0].x + (w - k * (b[1].x + b[0].x)) / two;
            let y = extent[0].y + (h - k * (b[1].y + b[0].y)) / two;

            let mut out = builder.clone();
            out.scale_set(one_five_zero * k);
            out.translate_set(&Coord { x, y });
            out
        },
        object,
    )
}

pub(super) fn fit_size_no_clip<B, CLIPC, CLIPU, PR, RC, RU, T>(
    builder: &B,
    size: Coord<T>,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<CLIPU, Bounds<T>, NoPCNU, PR, RU, Source<CLIPC, T>, T>,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Transform<T = T>,
    RC: Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    fit_extent_no_clip::<_, CLIPC, _, _, _, _, _>(
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

pub(super) fn fit_width_no_clip<B, CLIPC, CLIPU, PR, RC, RU, T>(
    builder: &B,
    width: T,
    object: &impl Streamable<T = T>,
) -> B
where
    B: Build<
            Projector<Bounds<T>> = Projector<CLIPU, Bounds<T>, NoPCNU, PR, RU, Source<CLIPC, T>, T>,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Transform<T = T>,
    RC: Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip::<_, CLIPC, _, _, _, _, _, _>(
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

pub(super) fn fit_height_no_clip<B, CLIPC, CLIPU, PR, RC, RU, T>(
    builder: &B,
    height: T,
    object: &impl Streamable<T = T>,
) -> B
where
    PR: Clone + Transform<T = T>,
    B: Build<
            Projector<Bounds<T>> = Projector<CLIPU, Bounds<T>, NoPCNU, PR, RU, Source<CLIPC, T>, T>,
        > + Clone
        + ScaleSet<T = T>
        + TranslateSet<T = T>,
    CLIPU: Clone + ConnectableClip<Output = CLIPC, SC = RC>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    RC: Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output<NoPCNC<Bounds<T>>> = RC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    let two = T::from(2.0_f64).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();

    fit_no_clip::<_, CLIPC, _, _, _, _, _, _>(
        builder,
        |b: [Coord<T>; 2], builder: &B| {
            let h = height;
            let k = h / (b[1].y - b[0].y);
            let x = -k * b[0].x;
            let y = (h - k * (b[1].y + b[0].y)) / two;

            let mut out = builder.clone();
            out.scale_set(one_five_zero * k)
                .translate_set(&Coord { x, y });
            out
        },
        object,
    )
}
