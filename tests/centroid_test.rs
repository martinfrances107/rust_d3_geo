#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod centroid_test {
    extern crate pretty_assertions;
    use geo::line_string;
    use geo::point;
    use geo::polygon;
    use geo::Geometry;
    use geo::GeometryCollection;
    use geo::MultiPoint;
    use geo::Point;
    use rust_d3_geo::centroid::stream::Stream as CentroidStream;
    use rust_d3_geo::data_object::sphere::Sphere;
    use rust_d3_geo::in_delta::in_delta_point;

    #[test]
    fn the_centroid_of_a_point_is_itself() {
        assert!(in_delta_point(
            CentroidStream::default().centroid(&Point::new(0f64, 0f64)),
            Point::new(0f64, 0f64),
            1e-6
        ));
        assert!(in_delta_point(
            CentroidStream::default().centroid(&Point::new(1f64, 1f64)),
            Point::new(1f64, 1f64),
            1e-6
        ));
        assert!(in_delta_point(
            CentroidStream::default().centroid(&Point::new(2f64, 3f64)),
            Point::new(2f64, 3f64),
            1e-6
        ));
        assert!(in_delta_point(
            CentroidStream::default().centroid(&Point::new(-4f64, -5f64)),
            Point::new(-4f64, -5f64),
            1e-6
        ));
    }

    #[test]
    fn centroid_sphereical_average() {
        println!(
            "the centroid of a set of points is the (spherical) average of its constituent members"
        );

        assert!(in_delta_point(
            CentroidStream::default().centroid(&GeometryCollection(vec![
                Geometry::Point(Point::new(0f64, 0f64)),
                Geometry::Point(Point::new(1f64, 2f64))
            ])),
            Point::new(0.499847, 1.000038),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&MultiPoint(vec![
                Point::new(0f64, 0f64),
                Point::new(1f64, 2f64),
            ])),
            point!(
                x: 0.499847f64,
                y: 1.000038f64
            ),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&MultiPoint(vec![
                Point::new(179f64, 0f64),
                Point::new(-179f64, 0f64),
            ])),
            point!( x: 180f64, y: 0f64 ),
            1e-6
        ));
    }

    //     // tape("the centroid of a set of points and their antipodes is ambiguous", function(test) {
    //     //   test.ok(d3.geoCentroid({type: "MultiPoint", coordinates: [[0, 0], [180, 0]]}).every(isNaN));
    //     //   test.ok(d3.geoCentroid({type: "MultiPoint", coordinates: [[0, 0], [90, 0], [180, 0], [-90, 0]]}).every(isNaN));
    //     //   test.ok(d3.geoCentroid({type: "MultiPoint", coordinates: [[0, 0], [0, 90], [180, 0], [0, -90]]}).every(isNaN));
    //     //   test.end();
    //     // });

    //     // tape("the centroid of the empty set of points is ambiguous", function(test) {
    //     //   test.ok(d3.geoCentroid({type: "MultiPoint", coordinates: []}).every(isNaN));
    //     //   test.end();
    //     // });

    #[test]
    fn line_string_great_arc_segments() {
        println!("the centroid of a line string is the (spherical) average of its constituent great arc segments");
        assert!(in_delta_point(
            CentroidStream::default().centroid(&line_string![(
                x: 0.0f64,
                y: 0.0f64
            ), (
                x: 1.0f64,
                y: 0.0f64
            )]),
            Point::new(0.5f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&line_string![
              ( x: 0.0f64, y: 0.0f64),
              ( x: 0f64, y: 90f64 )
            ]),
            Point::new(0f64, 45f64),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&line_string![
                ( x: 0f64, y: 0f64 ),
                ( x: 0f64, y: 45f64 ),
                ( x: 0f64, y: 90f64)
            ]),
            Point::new(0f64, 45f64),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&line_string![
                ( x: -1f64, y: -1f64 ), ( x: 1f64, y: 1f64 )
            ]),
            Point::new(0f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&line_string![
                ( x: -60f64, y: -1f64 ),
                ( x: 60f64, y: 1f64 ),
            ]),
            Point::new(0f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&line_string![
            ( x: 179f64, y: -1f64 ),
            ( x: -179f64, y: 1f64 ),
            ]),
            Point::new(180f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&line_string![
                (x: -179f64,  y: 0f64),
                (x: 0f64, y: 0f64),
                (x: 179f64, y: 0f64)
            ]),
            Point::new(0f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            CentroidStream::default().centroid(&line_string![
             ( x: -180f64, y: -90f64 ),
             ( x: 0f64, y: 0f64 ),
             ( x: 0f64, y: 90f64 ),
            ]),
            Point::new(0f64, 0f64),
            1e-6
        ));
    }

    //     // tape("the centroid of a great arc from a point to its antipode is ambiguous", function(test) {
    //     //   test.ok(d3.geoCentroid({type: "LineString", coordinates: [[180, 0], [0, 0]]}).every(isNaN));
    //     //   test.ok(d3.geoCentroid({type: "MultiLineString", coordinates: [[[0, -90], [0, 90]]]}).every(isNaN));
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a set of line strings is the (spherical) average of its constituent great arc segments", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "MultiLineString", coordinates: [[[0, 0], [0, 2]]]}), [0, 1], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("a line of zero length is treated as points", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "LineString", coordinates: [[1, 1], [1, 1]]}), [1, 1], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "GeometryCollection", geometries: [{type: "Point", coordinates: [0, 0]}, {type: "LineString", coordinates: [[1, 2], [1, 2]]}]}), [0.666534, 1.333408], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("an empty polygon with non-zero extent is treated as a line", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[1, 1], [2, 1], [3, 1], [2, 1], [1, 1]]]}), [2, 1.000076], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "GeometryCollection", geometries: [{type: "Point", coordinates: [0, 0]}, {type: "Polygon", coordinates: [[[1, 2], [1, 2], [1, 2], [1, 2]]]}]}), [0.799907, 1.600077], 1e-6);
    //     //   test.end();
    //     // });

    #[test]
    fn an_empty_polygon_with_non_zero_extent_is_treated_as_a_line() {
        println!("an empty polygon with non-zero extent is treated as a line");
        assert!(in_delta_point(
            CentroidStream::default().centroid(&polygon![
            (
                x: 1.0f64,
                y: 1.0f64
            ),
            (
                x: 2.0f64,
                y: 1.0f64
            ),
            (
                x: 3.0f64,
                y: 1.0f64
            ),
            (
                x: 2.0f64,
                y: 1.0f64
            ),
            (
                x: 1.0f64,
                y: 1.0f64
            )
            ]),
            Point::new(2f64, 1.000076f64),
            1e-6
        ));
    }

    //     // tape("an empty polygon with zero extent is treated as a point", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[1, 1], [1, 1], [1, 1], [1, 1]]]}), [1, 1], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "GeometryCollection", geometries: [{type: "Point", coordinates: [0, 0]}, {type: "Polygon", coordinates: [[[1, 2], [1, 2], [1, 2], [1, 2]]]}]}), [0.799907, 1.600077], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of the equator is ambiguous", function(test) {
    //     //   test.ok(d3.geoCentroid({type: "LineString", coordinates: [[0, 0], [120, 0], [-120, 0], [0, 0]]}).every(isNaN));
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a polygon is the (spherical) average of its surface", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -90], [0, 0], [0, 90], [1, 0], [0, -90]]]}), [0.5, 0], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [array.range(-180, 180 + 1 / 2, 1).map(function(x) { return [x, -60]; })]})[1], -90, 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -10], [0, 10], [10, 10], [10, -10], [0, -10]]]}), [5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a set of polygons is the (spherical) average of its surface", function(test) {
    //     //   var circle = d3.geoCircle();
    //     //   test.inDelta(d3.geoCentroid({
    //     //     type: "MultiPolygon",
    //     //     coordinates: [
    //     //       circle.radius(45).center([90, 0])().coordinates,
    //     //       circle.radius(60).center([-90, 0])().coordinates
    //     //     ]
    //     //   }), [-90, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a lune is the (spherical) average of its surface", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -90], [0, 0], [0, 90], [1, 0], [0, -90]]]}), [0.5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: 5°", function(test) {
    //     //   test.inDelta(d3.geoCentroid(d3.geoCircle().radius(5).center([30, 45])()), [30, 45], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: 135°", function(test) {
    //     //   test.inDelta(d3.geoCentroid(d3.geoCircle().radius(135).center([30, 45])()), [30, 45], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: South Pole", function(test) {
    //     //   test.equal(d3.geoCentroid({type: "Polygon", coordinates: [array.range(-180, 180 + 1 / 2, 1).map(function(x) { return [x, -60]; })]})[1], -90);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: equator", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -10], [0, 10], [10, 10], [10, -10], [0, -10]]]}), [5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: equator with coincident points", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -10], [0, 10], [0, 10], [10, 10], [10, -10], [0, -10]]]}), [5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: other", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[-180, 0], [-180, 10], [-179, 10], [-179, 0], [-180, 0]]]}), [-179.5, 4.987448], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: concentric rings", function(test) {
    //     //   var circle = d3.geoCircle().center([0, 45]),
    //     //       coordinates = circle.radius(60)().coordinates;
    //     //   coordinates.push(circle.radius(45)().coordinates[0].reverse());
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: coordinates}), [0, 45], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a spherical square on the equator", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -10], [0, 10], [10, 10], [10, -10], [0, -10]]]}), [5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a spherical square touching the antimeridian", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[-180, 0], [-180, 10], [-179, 10], [-179, 0], [-180, 0]]]}), [-179.5, 4.987448], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("concentric rings", function(test) {
    //     //   var circle = d3.geoCircle().center([0, 45]),
    //     //       coordinates = circle.radius(60)().coordinates;
    //     //   coordinates.push(circle.radius(45)().coordinates[0].reverse());
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: coordinates}), [0, 45], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a sphere is ambiguous", function(test) {
    //     //   test.ok(d3.geoCentroid({type: "Sphere"}).every(isNaN));
    //     //   test.end();
    //     // });
    #[test]
    fn the_centroid_of_a_sphere_is_ambigous() {
        println!("the centroid of a sphere is ambiguous");
        let point: Point<f64> = CentroidStream::default().centroid(&Sphere::default());
        assert!(point.x().is_nan());
        assert!(point.y().is_nan());
    }
    //     // tape("the centroid of a feature is the centroid of its constituent geometry", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Feature", geometry: {type: "LineString", coordinates: [[1, 1], [1, 1]]}}), [1, 1], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "Feature", geometry: {type: "Point", coordinates: [1, 1]}}), [1, 1], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "Feature", geometry: {type: "Polygon", coordinates: [[[0, -90], [0, 0], [0, 90], [1, 0], [0, -90]]]}}), [0.5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a feature collection is the centroid of its constituent geometry", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "FeatureCollection", features: [
    //     //     {type: "Feature", geometry: {type: "LineString", coordinates: [[179, 0], [180, 0]]}},
    //     //     {type: "Feature", geometry: {type: "Point", coordinates: [0, 0]}}
    //     //   ]}), [179.5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a non-empty line string and a point only considers the line string", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "LineString", coordinates: [[179, 0], [180, 0]]},
    //     //     {type: "Point", coordinates: [0, 0]}
    //     //   ]}), [179.5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a non-empty polygon, a non-empty line string and a point only considers the polygon", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "Polygon", coordinates: [[[-180, 0], [-180, 1], [-179, 1], [-179, 0], [-180, 0]]]},
    //     //     {type: "LineString", coordinates: [[179, 0], [180, 0]]},
    //     //     {type: "Point", coordinates: [0, 0]}
    //     //   ]}), [-179.5, 0.500006], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "Point", coordinates: [0, 0]},
    //     //     {type: "LineString", coordinates: [[179, 0], [180, 0]]},
    //     //     {type: "Polygon", coordinates: [[[-180, 0], [-180, 1], [-179, 1], [-179, 0], [-180, 0]]]}
    //     //   ]}), [-179.5, 0.500006], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of the sphere and a point is the point", function(test) {
    //     //   test.deepEqual(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "Sphere"},
    //     //     {type: "Point", coordinates: [0, 0]}
    //     //   ]}), [0, 0]);
    //     //   test.deepEqual(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "Point", coordinates: [0, 0]},
    //     //     {type: "Sphere"}
    //     //   ]}), [0, 0]);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a detailed feature is correct", function(test) {
    //     //   var ny = require("./data/ny.json");
    //     //   test.inDelta(d3.geoCentroid(ny), [-73.93079, 40.69447], 1e-5);
    //     //   test.end();
    //     // });
}
