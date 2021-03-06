use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;

use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::data_object::DataObject;
use crate::path::bounds_stream::BoundsStream;
use crate::path::PathResult;
use crate::path::PathResultEnum;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::projection::Projection;
use super::projection_mutator::ProjectionMutator;

fn fit<PR, T>(
    projection: ProjectionMutator<PR, T>,
    fit_bounds: Box<
        dyn FnOnce([Coordinate<T>; 2], ProjectionMutator<PR, T>) -> ProjectionMutator<PR, T>,
    >,
    object: DataObject<T>,
) -> ProjectionMutator<PR, T>
where
    PR: Clone + Default + Transform<TcC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    let clip = projection.get_clip_extent();
    let projection1 = projection
        .scale(T::from(150.0).unwrap())
        .translate(&Coordinate {
            x: T::zero(),
            y: T::zero(),
        });
    let projection2 = match clip {
        Some(_) => projection1.clip_extent(None),
        None => projection1,
    };

    let bounds_stream = StreamDst::BS(BoundsStream::default());
    let mut stream_in = projection2.stream(bounds_stream);

    object.to_stream(&mut stream_in);
    let bounds = match stream_in.get_dst().result() {
        Some(PathResultEnum::Bounds(bounds)) => bounds,
        _ => {
            panic!("Expecting only a bounds result from a Bounds stream.");
        }
    };
    let projection3 = fit_bounds(bounds, projection2);
    match clip {
        Some(_) => projection3.clip_extent(clip),
        None => projection3,
    }
}

pub fn fit_extent<PR, T>(
    projection: ProjectionMutator<PR, T>,
    extent: [Coordinate<T>; 2],
    object: DataObject<T>,
) -> ProjectionMutator<PR, T>
where
    PR: Transform<TcC = Coordinate<T>> + Clone + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    fit(
        projection,
        Box::new(
            move |b: [Coordinate<T>; 2], projection: ProjectionMutator<PR, T>| {
                let two = T::from(2.0).unwrap();
                let w = extent[1].x - extent[0].y;
                let h = extent[1].y - extent[0].y;
                let k = (w / (b[1].x - b[0].x)).min(h / (b[0].y - b[0].y));
                let x = extent[0].x + (w - k * (b[1].x + b[0].x)) / two;
                let y = extent[0].y + (h - k * (b[1].y + b[0].y)) / two;

                let projection_out = projection
                    .scale(T::from(150.0).unwrap() * k)
                    .translate(&Coordinate { x: x, y: y });
                projection_out
            },
        ),
        object,
    )
}
// export function fitExtent(projection, extent, object) {
//   return fit(projection, function(b) {
//     var w = extent[1][0] - extent[0][0],
//         h = extent[1][1] - extent[0][1],
//         k = Math.min(w / (b[1][0] - b[0][0]), h / (b[1][1] - b[0][1])),
//         x = +extent[0][0] + (w - k * (b[1][0] + b[0][0])) / 2,
//         y = +extent[0][1] + (h - k * (b[1][1] + b[0][1])) / 2;
//     projection.scale(150 * k).translate([x, y]);
//   }, object);
// }

// export function fitSize(projection, size, object) {
//   return fitExtent(projection, [[0, 0], size], object);
// }

// export function fitWidth(projection, width, object) {
//   return fit(projection, function(b) {
//     var w = +width,
//         k = w / (b[1][0] - b[0][0]),
//         x = (w - k * (b[1][0] + b[0][0])) / 2,
//         y = -k * b[0][1];
//     projection.scale(150 * k).translate([x, y]);
//   }, object);
// }

// export function fitHeight(projection, height, object) {
//   return fit(projection, function(b) {
//     var h = +height,
//         k = h / (b[1][1] - b[0][1]),
//         x = -k * b[0][0],
//         y = (h - k * (b[1][1] + b[0][1])) / 2;
//     projection.scale(150 * k).translate([x, y]);
//   }, object);
// }
