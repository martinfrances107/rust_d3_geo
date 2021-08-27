// use crate::projection::builder::Builder;
use num_traits::AsPrimitive;

use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;

use num_traits::FloatConst;

use crate::data_object::DataObject;
use crate::path::bounds_stream::BoundsStream;
use crate::path::Result;
use crate::path::ResultEnum;
use crate::projection::builder::Builder;
use crate::projection::scale::Scale;
use crate::projection::Line;
use crate::projection::PointVisible;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::stream::Streamable;

use super::clip_extent::ClipExtent;
// use super::projection_trait::ProjectionTrait;
use super::translate::Translate;

type FitBounds<DRAIN, L, PR, PV, T> = Box<
    dyn FnOnce([Coordinate<T>; 2], Builder<DRAIN, L, PR, PV, T>) -> Builder<DRAIN, L, PR, PV, T>,
>;

fn fit<L, PR, PV, T>(
    builder: Builder<BoundsStream<T>, L, PR, PV, T>,
    fit_bounds: FitBounds<BoundsStream<T>, L, PR, PV, T>,
    object: DataObject<T>,
) -> Builder<BoundsStream<T>, L, PR, PV, T>
where
    // DRAIN: Stream<T = T> + Default,
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

pub fn fit_extent<L, PR, PV, T>(
    builder: Builder<BoundsStream<T>, L, PR, PV, T>,
    extent: [Coordinate<T>; 2],
    object: DataObject<T>,
) -> Builder<BoundsStream<T>, L, PR, PV, T>
where
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    fit(
        builder,
        Box::new(
            move |b: [Coordinate<T>; 2], builder: Builder<BoundsStream<T>, L, PR, PV, T>| {
                let two = T::from(2.0).unwrap();
                let w = extent[1].x - extent[0].y;
                let h = extent[1].y - extent[0].y;
                let k = (w / (b[1].x - b[0].x)).min(h / (b[0].y - b[0].y));
                let x = extent[0].x + (w - k * (b[1].x + b[0].x)) / two;
                let y = extent[0].y + (h - k * (b[1].y + b[0].y)) / two;

                builder
                    .scale(T::from(150.0).unwrap() * k)
                    .translate(&Coordinate { x: x, y: y })
            },
        ),
        object,
    )
}

// // // export function fitExtent(projection, extent, object) {
// // //   return fit(projection, function(b) {
// // //     var w = extent[1][0] - extent[0][0],
// // //         h = extent[1][1] - extent[0][1],
// // //         k = Math.min(w / (b[1][0] - b[0][0]), h / (b[1][1] - b[0][1])),
// // //         x = +extent[0][0] + (w - k * (b[1][0] + b[0][0])) / 2,
// // //         y = +extent[0][1] + (h - k * (b[1][1] + b[0][1])) / 2;
// // //     projection.scale(150 * k).translate([x, y]);
// // //   }, object);
// // // }

// // // export function fitSize(projection, size, object) {
// // //   return fitExtent(projection, [[0, 0], size], object);
// // // }

// // // export function fitWidth(projection, width, object) {
// // //   return fit(projection, function(b) {
// // //     var w = +width,
// // //         k = w / (b[1][0] - b[0][0]),
// // //         x = (w - k * (b[1][0] + b[0][0])) / 2,
// // //         y = -k * b[0][1];
// // //     projection.scale(150 * k).translate([x, y]);
// // //   }, object);
// // // }

// // // export function fitHeight(projection, height, object) {
// // //   return fit(projection, function(b) {
// // //     var h = +height,
// // //         k = h / (b[1][1] - b[0][1]),
// // //         x = -k * b[0][0],
// // //         y = (h - k * (b[1][1] + b[0][1])) / 2;
// // //     projection.scale(150 * k).translate([x, y]);
// // //   }, object);
// // // }
