use geo_types::Coord;

use crate::math::EPSILON;
use crate::projection::ClipExtentSet;
use crate::projection::ScaleGet;
use crate::projection::TranslateSet;

use super::Builder;

impl<DRAIN> TranslateSet for Builder<DRAIN>
where
    DRAIN: Clone,
{
    type T = f64;

    fn translate_set(&mut self, t: &Coord<f64>) -> &mut Self {
        let k = self.pr.lower_48.scale();

        self.pr.lower_48_point = self.pr.lower_48.translate_set(t).clip_extent_set(&[
            Coord {
                x: t.x - 0.455_f64 * k,
                y: t.y - 0.234 * k,
            },
            Coord {
                x: t.x + 0.455_f64 * k,
                y: t.y + 0.234 * k,
            },
        ]);

        self.pr.alaska_point = self
            .pr
            .alaska
            .translate_set(&Coord {
                x: t.x - 0.307_f64 * k,
                y: t.y - 0.201 * k,
            })
            .clip_extent_set(&[
                Coord {
                    x: t.x - 0.425_f64 * k + EPSILON,
                    y: t.y - 0.120 * k + EPSILON,
                },
                Coord {
                    x: t.x - 0.214_f64 * k - EPSILON,
                    y: t.y - 0.234 * k - EPSILON,
                },
            ]);

        self.pr.hawaii_point = self
            .pr
            .hawaii
            .translate_set(&Coord {
                x: t.x - 0.205_f64 * k,
                y: t.y - 0.212 * k,
            })
            .clip_extent_set(&[
                Coord {
                    x: t.x - 0.214_f64 * k + EPSILON,
                    y: t.y - 0.166 * k + EPSILON,
                },
                Coord {
                    x: t.x - 0.214 * k + EPSILON,
                    y: t.y + 0.234 * k - EPSILON,
                },
            ]);
        self
    }
}
