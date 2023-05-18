use geo::CoordFloat;
use geo_types::Coord;

use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, PCNU, PR, RU, T> Transform for Builder<CLIPC, CLIPU, PCNU, PR, RU, T>
where
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
        let inverted = self.project_transform.invert(p);
        Coord {
            x: inverted.x.to_degrees(),
            y: inverted.y.to_degrees(),
        }
    }
}
