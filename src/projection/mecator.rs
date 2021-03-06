use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::Transform;

use super::projection::Projection;
use super::projection_mutator::ProjectionMutator;

#[derive(Clone, Copy, Debug, Default)]
pub struct MecatorRaw<T>
where
    T: CoordFloat + Default,
{
    phantom: PhantomData<T>,
}

impl<T> MecatorRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pub fn gen_projection_mutator() -> ProjectionMutator<MecatorRaw<T>, T> {
        let tau = T::from(2).unwrap() * T::PI();
        let o = MecatorRaw::default();
        ProjectionMutator::from_projection_raw(o, None).scale(T::from(961).unwrap() / tau)
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Transform
    for MecatorRaw<T>
{
    type TcC = Coordinate<T>;
    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let two = T::from(2).unwrap();
        Coordinate {
            x: p.x,
            y: ((T::FRAC_PI_2() + p.y) / two).tan().ln(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let two = T::from(2).unwrap();
        Coordinate {
            x: p.x,
            y: two * (p.y.exp()).atan() - T::FRAC_PI_2(),
        }
    }
}

// export function mercatorRaw(lambda, phi) {
//   return [lambda, log(tan((halfPi + phi) / 2))];
// }

// mercatorRaw.invert = function(x, y) {
//   return [x, 2 * atan(exp(y)) - halfPi];
// };

// export default function() {
//   return mercatorProjection(mercatorRaw)
//       .scale(961 / tau);
// }

// export function mercatorProjection(project) {
//   var m = projection(project),
//       center = m.center,
//       scale = m.scale,
//       translate = m.translate,
//       clipExtent = m.clipExtent,
//       x0 = null, y0, x1, y1; // clip extent

//   m.scale = function(_) {
//     return arguments.length ? (scale(_), reclip()) : scale();
//   };

//   m.translate = function(_) {
//     return arguments.length ? (translate(_), reclip()) : translate();
//   };

//   m.center = function(_) {
//     return arguments.length ? (center(_), reclip()) : center();
//   };

//   m.clipExtent = function(_) {
//     return arguments.length ? ((_ == null ? x0 = y0 = x1 = y1 = null : (x0 = +_[0][0], y0 = +_[0][1], x1 = +_[1][0], y1 = +_[1][1])), reclip()) : x0 == null ? null : [[x0, y0], [x1, y1]];
//   };

//   function reclip() {
//     var k = pi * scale(),
//         t = m(rotation(m.rotate()).invert([0, 0]));
//     return clipExtent(x0 == null
//         ? [[t[0] - k, t[1] - k], [t[0] + k, t[1] + k]] : project === mercatorRaw
//         ? [[Math.max(t[0] - k, x0), y0], [Math.min(t[0] + k, x1), y1]]
//         : [[x0, Math.max(t[1] - k, y0)], [x1, Math.min(t[1] + k, y1)]]);
//   }

//   return reclip();
// }
