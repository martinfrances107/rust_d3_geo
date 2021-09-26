// use crate::projection::builder::Builder;
use num_traits::AsPrimitive;
use num_traits::Float;

use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::data_object::DataObject;
use crate::path::bounds_stream::BoundsStream;
use crate::path::Result;
use crate::path::ResultEnum;
use crate::stream::Streamable;

use super::builder::Builder;
use super::Line;
use super::PointVisible;
use super::Raw as ProjectionRaw;
use super::Scale;
use super::ClipExtent;
use super::Translate;

type FitBounds<DRAIN, L, PR, PV, T> = Box<
    dyn FnOnce([Coordinate<T>; 2], Builder<DRAIN, L, PR, PV, T>) -> Builder<DRAIN, L, PR, PV, T>,
>;

fn fit<L, PR, PV, T>(
    builder: Builder<BoundsStream<T>, L, PR, PV, T>,
    fit_bounds: FitBounds<BoundsStream<T>, L, PR, PV, T>,
    object: DataObject<T>,
) -> Builder<BoundsStream<T>, L, PR, PV, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    let clip = builder.get_clip_extent();
    let builder1 = builder
        .scale(T::from(150.0).unwrap())
        .translate(&Coordinate {
            x: T::zero(),
            y: T::zero(),
        });

    let builder2 = match clip {
        Some(_) => builder1.clip_extent(None),
        None => builder1,
    };

    let bounds_stream = Rc::new(RefCell::new(BoundsStream::default()));
    let mut stream_in = builder2.build().stream(bounds_stream.clone());

    object.to_stream(&mut stream_in);
    let result = bounds_stream.borrow_mut().result();
    let bounds = match result {
        Some(ResultEnum::Bounds(bounds)) => bounds,
        _ => {
            panic!("Expecting only a bounds result from a Bounds stream.");
        }
    };
    let builder3 = fit_bounds(bounds, builder2);
    match clip {
        Some(_) => builder3.clip_extent(clip),
        None => builder3,
    }
}

pub(super) fn fit_extent<L, PR, PV, T>(
    builder: Builder<BoundsStream<T>, L, PR, PV, T>,
    extent: [[T; 2]; 2],
    object: DataObject<T>,
) -> Builder<BoundsStream<T>, L, PR, PV, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    let two = T::from(2.0).unwrap();
    fit(
        builder,
        Box::new(
            move |b: [Coordinate<T>; 2], builder: Builder<BoundsStream<T>, L, PR, PV, T>| {
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

pub(super) fn fit_size<L, PR, PV, T>(
    builder: Builder<BoundsStream<T>, L, PR, PV, T>,
    size: [T; 2],
    object: DataObject<T>,
) -> Builder<BoundsStream<T>, L, PR, PV, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    fit_extent(builder, [[T::zero(), T::zero()], size], object)
}
