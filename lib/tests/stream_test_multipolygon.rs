extern crate pretty_assertions;

use geo::polygon;
use geo::MultiPolygon;
use geo_types::Coord;

use d3_geo_rs::stream::Stream;
use d3_geo_rs::stream::Streamable;

#[derive(Clone, Debug, Default)]
struct MultiPolygonStream {
    calls: u8,
    coordinates: f64,
}

impl Stream for MultiPolygonStream {
    type EP = MultiPolygonStream;
    type T = f64;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    fn line_end(&mut self) {
        self.calls += 1;
        assert!(self.calls == 5 || self.calls == 11);
    }

    fn line_start(&mut self) {
        self.calls += 1;
        assert!(self.calls == 2 || self.calls == 8)
    }

    fn point(&mut self, p: &Coord<f64>, _m: Option<u8>) {
        self.coordinates += 1_f64;
        assert!(p.x == self.coordinates);
        self.coordinates += 1_f64;
        assert!(p.y == self.coordinates);
        self.coordinates += 1_f64;
        // assert!(m.unwrap() == self.coordinates as u8);
        self.calls += 1;
        assert!(
            3 <= self.calls && self.calls <= 4
                || 9 <= self.calls && self.calls <= 10
        );
    }

    fn polygon_end(&mut self) {
        self.calls += 1;
        assert!(self.calls == 6 || self.calls == 12);
    }

    fn polygon_start(&mut self) {
        self.calls += 1;
        assert!(self.calls == 1 || self.calls == 7)
    }
}

#[test]
fn multi_polygon() {
    println!(
            "geoStream(MultiPolygon) -> (polygonStart, lineStart, point, lineEnd, polygonEnd)*"
        );

    let mut stream = MultiPolygonStream::default();
    let mp = MultiPolygon(vec![
        polygon![
            (x:1_f64, y:2_f64),
            (x:4_f64, y:5_f64),
            (x:1_f64, y:2_f64),
        ],
        polygon![
            (x: 7_f64, y: 8_f64),
            (x: 10_f64, y: 11_f64),
            (x: 7_f64, y: 8_f64)
        ],
    ]);

    mp.to_stream(&mut stream);
    assert!(stream.calls == 12);
}
