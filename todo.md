## TODO

TODDAY

index_test
invert_test
mercator test ( mostly reinstated.)
snapshot test
fit_test -- needs works.
path_bounds_test
projection_rotate_test -- did it ever work?
snapshot

* Implement remaining fit_tests --- gives a minor bump in code coverage.

* examples/projections
* examples/rings

 Investigiate why mercator fails.
  -- mercator has differences between f64 and f32.

* examples/graticule - does this example need to exist?
when projection displays graticules.
styling of globe should include graticules?

IF I am keeping it .. then
- Convert to typescript
- Stop using cdn as source of d3 module.
- draw a clear distinction between what is JS and what is rust.

* Write examples/index.html -- a guide to building rust projects.

* Develop styling for world maps graticules on bottom, overlayed by land.

* Add examples landing page. - showing images of the each projection.

* Check on performance of real world test ... ortho projection of 50m.json altas
 with graticules showing africa or South pacific.

* Put images of ring and world map on homepage. Plus add a gallery homepage for the examples
with links to the more detailed mini-apps.

* Review  path/centroid_test.rs

* Refactor trait names ending in Trait. LineTrait, StreamTrait, PointRadiusTrait.

* Detail to development of mocks for RenderingContext2d

* Examples projections CSS styling needs minor adjustment font sizes of text
   looks off ..select/apply a simple responsive css template.
  ( eg.  Boilerplate, material-ui)

##  Before API lock-down

* Detail why Feature and Feactue collection is complicated.

* Define graticule10 as an alias for Graticule::default().
 -- the code in examples looks scrappy.

* Justify whey PathResult is not implemented for LengthStream? is cal() a deviation from the interface.

* Same arguement for CicleStream.

## Not Yet Implemented

* src/projection/albers.js
* src/projection/albersUsa.js
* src/projection/conic.js
* src/projection/conicConformal
* src/projection/conicEqualArea
* src/projection/conicEquidistant
* src/projection/cylindricalEqualArea
* src/projection/equalEarth
* src/interpolate.js
* src/projection/natualEarth.js

## The State of Testing

snapshot tests are missing.

src/rotation ?

area_test - Some tests not yet implemented. Exposes bug, also not complete.

bounds_test - src or test not yet implemented.

centroid_test - Needs minor additions

clip_circle_test - Needs regex development.. Path::API has changed since this last compiled.

contains_test - src and test not implmented.

distance_test - Complete.

equirectangular_test - Complete.

index_test - Need to get mocks working.

length_test -  Needs feature collection tests.

mercator_test - Bugs lurking .. see commented out tests.

projection_angle_test  - Needs geoIdentity test

path_area_test - Complete.

path_bounds_test - Complete.

path_string_test - Need to port test for Features, FeatueCollection, GeometryCollection, Polygon and LineString.

polygon_contains_test - Complete.

projection_reflect_test - Complete.

stereographic_test - Complete.
