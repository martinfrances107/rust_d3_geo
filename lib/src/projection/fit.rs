use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::post_clip_node::PostClipNode;
use crate::clip::Line;
use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::stream::Stream;
use crate::stream::Streamable;

use super::builder::Builder;
use super::resample::ResampleNode;
use super::stream_node::StreamNode;
use super::ClipExtent;
use super::PointVisible;
use super::Raw as ProjectionRaw;
use super::Scale;
use super::Translate;

type FitBounds<DRAIN, LINE, PR, PV, T> = Box<
    dyn Fn([Coordinate<T>; 2], Builder<DRAIN, LINE, PR, PV, T>) -> Builder<DRAIN, LINE, PR, PV, T>,
>;

fn fit<LINE, PR, PV, T>(
    builder: Builder<Bounds<T>, LINE, PR, PV, T>,
    fit_bounds: FitBounds<Bounds<T>, LINE, PR, PV, T>,
    object: &impl Streamable<T = T>,
) -> Builder<Bounds<T>, LINE, PR, PV, T>
where
    LINE: Line,
    StreamNode<
        Bounds<T>,
        LINE,
        ResampleNode<Bounds<T>, PR, PostClipNode<Bounds<T>, Bounds<T>, T>, T>,
        T,
    >: Stream<EP = Bounds<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let clip = builder.get_clip_extent();
    let builder1 = builder
        .scale(T::from(150.0).unwrap())
        .translate(&Coordinate {
            x: T::zero(),
            y: T::zero(),
        });

    let builder2 = match clip {
        Some(_) => builder1.clip_extent_clear(),
        None => builder1,
    };

    let bounds_stream = Bounds::default();
    let mut stream_in = builder2.build().stream(bounds_stream);

    object.to_stream(&mut stream_in);
    let bounds = stream_in.get_endpoint().result();
    let builder3 = fit_bounds(bounds, builder2);
    match clip {
        Some(extent) => builder3.clip_extent(&extent),
        None => builder3,
    }
}

pub(super) fn fit_extent<LINE, PR, PV, T>(
    builder: Builder<Bounds<T>, LINE, PR, PV, T>,
    extent: [[T; 2]; 2],
    object: &impl Streamable<T = T>,
) -> Builder<Bounds<T>, LINE, PR, PV, T>
where
    LINE: Line,
    StreamNode<
        Bounds<T>,
        LINE,
        ResampleNode<Bounds<T>, PR, PostClipNode<Bounds<T>, Bounds<T>, T>, T>,
        T,
    >: Stream<EP = Bounds<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let two = T::from(2.0).unwrap();
    let one_five_zero = T::from(150_f64).unwrap();
    fit(
        builder,
        Box::new(
            move |b: [Coordinate<T>; 2], builder: Builder<Bounds<T>, LINE, PR, PV, T>| {
                let w = extent[1][0] - extent[0][0];
                let h = extent[1][1] - extent[0][1];
                let k = Float::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
                let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
                let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

                builder
                    .scale(one_five_zero * k)
                    .translate(&Coordinate { x, y })
            },
        ),
        object,
    )
}

pub(super) fn fit_size<LINE, PR, PV, T>(
    builder: Builder<Bounds<T>, LINE, PR, PV, T>,
    size: [T; 2],
    object: &impl Streamable<T = T>,
) -> Builder<Bounds<T>, LINE, PR, PV, T>
where
    LINE: Line,
    StreamNode<
        Bounds<T>,
        LINE,
        ResampleNode<Bounds<T>, PR, PostClipNode<Bounds<T>, Bounds<T>, T>, T>,
        T,
    >: Stream<EP = Bounds<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    fit_extent(builder, [[T::zero(), T::zero()], size], object)
}

pub(super) fn fit_width<LINE, PR, PV, T>(
    builder: Builder<Bounds<T>, LINE, PR, PV, T>,
    w: T,
    object: &impl Streamable<T = T>,
) -> Builder<Bounds<T>, LINE, PR, PV, T>
where
    LINE: Line,
    StreamNode<
        Bounds<T>,
        LINE,
        ResampleNode<Bounds<T>, PR, PostClipNode<Bounds<T>, Bounds<T>, T>, T>,
        T,
    >: Stream<EP = Bounds<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let two = T::from(2.0).unwrap();
    let one_five_zero = T::from(150).unwrap();
    fit(
        builder,
        Box::new(
            move |b: [Coordinate<T>; 2], builder: Builder<Bounds<T>, LINE, PR, PV, T>| {
                let k = w / (b[1].x - b[0].x);
                dbg!(b, k, w);
                let x = (w - k * (b[1].x - b[0].x)) / two;
                let y = -k * b[0].y;

                builder
                    .scale(one_five_zero * k)
                    .translate(&Coordinate { x, y })
            },
        ),
        object,
    )
}

pub(super) fn fit_height<LINE, PR, PV, T>(
    builder: Builder<Bounds<T>, LINE, PR, PV, T>,
    h: T,
    object: &impl Streamable<T = T>,
) -> Builder<Bounds<T>, LINE, PR, PV, T>
where
    LINE: Line,
    StreamNode<
        Bounds<T>,
        LINE,
        ResampleNode<Bounds<T>, PR, PostClipNode<Bounds<T>, Bounds<T>, T>, T>,
        T,
    >: Stream<EP = Bounds<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let two = T::from(2.0).unwrap();
    let one_five_zero = T::from(150).unwrap();
    fit(
        builder,
        Box::new(
            move |b: [Coordinate<T>; 2], builder: Builder<Bounds<T>, LINE, PR, PV, T>| {
                let k = h / (b[1].y - b[0].y);
                let x = -k * b[0].x;
                let y = (h - k * (b[1].y - b[0].y)) / two;

                builder
                    .scale(one_five_zero * k)
                    .translate(&Coordinate { x, y })
            },
        ),
        object,
    )
}
