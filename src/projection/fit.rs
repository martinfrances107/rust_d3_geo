use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::data_object::DataObject;
use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::path::ResultEnum;
use crate::stream::Streamable;

use super::builder::Builder;
use super::ClipExtent;
use super::PointVisible;
use super::Raw as ProjectionRaw;
use super::Scale;
use super::Translate;

type FitBounds<DRAIN, EP, PR, PV, T> = Box<
    dyn FnOnce([Coordinate<T>; 2], Builder<DRAIN, EP, PR, PV, T>) -> Builder<DRAIN, EP, PR, PV, T>,
>;

fn fit<PR, PV, T>(
    builder: Builder<Bounds<T>, Bounds<T>, PR, PV, T>,
    fit_bounds: FitBounds<Bounds<T>, Bounds<T>, PR, PV, T>,
    object: &DataObject<T>,
) -> Builder<Bounds<T>, Bounds<T>, PR, PV, T>
where
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

    let mut bounds_stream = Bounds::default();
    let mut stream_in = builder2.build().stream(bounds_stream.clone());

    object.to_stream(&mut stream_in);
    let result = bounds_stream.result();
    let bounds = match result {
        Some(ResultEnum::Bounds(bounds)) => bounds,
        _ => {
            panic!("Expecting only a bounds result from a Bounds stream.");
        }
    };
    let builder3 = fit_bounds(bounds, builder2);
    match clip {
        Some(extent) => builder3.clip_extent(&extent),
        None => builder3,
    }
}

pub(super) fn fit_extent<PR, PV, T>(
    builder: Builder<Bounds<T>, Bounds<T>, PR, PV, T>,
    extent: [[T; 2]; 2],
    object: &DataObject<T>,
) -> Builder<Bounds<T>, Bounds<T>, PR, PV, T>
where
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let two = T::from(2.0).unwrap();
    fit(
        builder,
        Box::new(
            move |b: [Coordinate<T>; 2], builder: Builder<Bounds<T>, PR, PV, T>| {
                let w = extent[1][0] - extent[0][0];
                let h = extent[1][1] - extent[0][1];
                let k = Float::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
                let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
                let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

                builder
                    .scale(T::from(150.0).unwrap() * k)
                    .translate(&Coordinate { x, y })
            },
        ),
        object,
    )
}

pub(super) fn fit_size<PR, PV, T>(
    builder: Builder<Bounds<T>, Bounds<T>, PR, PV, T>,
    size: [T; 2],
    object: &DataObject<T>,
) -> Builder<Bounds<T>, Bounds<T>, PR, PV, T>
where
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    fit_extent(builder, [[T::zero(), T::zero()], size], object)
}
