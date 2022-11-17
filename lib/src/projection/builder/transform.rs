use geo::CoordFloat;
use geo_types::Coord;

use crate::Transform;

use super::Builder;

impl<DRAIN, CLIPC, CLIPU, PCNU, PR, RC, RU, T> Transform
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    PR: Transform<T = T>,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let pt = Coord {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        };
        self.project_transform.transform(&pt)
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        let pt = Coord {
            x: p.x.to_degrees(),
            y: p.y.to_degrees(),
        };
        self.project_transform.invert(&pt)
    }
}
