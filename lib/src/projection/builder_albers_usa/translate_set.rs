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

        self.pr.lower_48.translate_set(t).clip_extent_set(&[
            Coord {
                x: 0.455_f64.mul_add(-k, t.x),
                y: 0.234f64.mul_add(-k, t.y),
            },
            Coord {
                x: 0.455_f64.mul_add(k, t.x),
                y: 0.234f64.mul_add(k, t.y),
            },
        ]);

        self.pr
            .alaska
            .translate_set(&Coord {
                x: 0.307_f64.mul_add(-k, t.x),
                y: 0.201f64.mul_add(-k, t.y),
            })
            .clip_extent_set(&[
                Coord {
                    x: 0.425_f64.mul_add(-k, t.x) + EPSILON,
                    y: 0.120f64.mul_add(-k, t.y) + EPSILON,
                },
                Coord {
                    x: 0.214_f64.mul_add(-k, t.x) - EPSILON,
                    y: 0.234f64.mul_add(-k, t.y) - EPSILON,
                },
            ]);

        self.pr
            .hawaii
            .translate_set(&Coord {
                x: 0.205_f64.mul_add(-k, t.x),
                y: 0.212f64.mul_add(-k, t.y),
            })
            .clip_extent_set(&[
                Coord {
                    x: 0.214_f64.mul_add(-k, t.x) + EPSILON,
                    y: 0.166f64.mul_add(-k, t.y) + EPSILON,
                },
                Coord {
                    x: 0.214f64.mul_add(-k, t.x) + EPSILON,
                    y: 0.234f64.mul_add(k, t.y) - EPSILON,
                },
            ]);
        self
    }
}
