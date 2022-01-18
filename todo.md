## TODO

* Implement remaining fit_tests --- gives a minor bump in code coverage.

* write example/index.html -- a guide to building rust projects.

* Develop styling for world maps graticules on bottom, overlayed by land.

* Add examples landing page. - showing images of the each projection.

* Check on performance of real world test ... ortho projection of 50m.json altas
 with graticules showing africa or South pacific.

* Put images of ring and world map on homepage. Plus add a gallery homepage for the examples
with links to the more detailed mini-apps.

* Review  path/centroid_test.rs

* Refactor trait names ending in Trait. LineTrait, StreamTrait, PointRadiusTrait.

* Detail why Feature and Feactue collection is complicated.

* Detail to development of mocks for RenderingContext2d

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

src/rotation

area_test - Some tests not yet implemented. Exposes bug, also not complete.

bounds_test - src or test not yet implemented.

centroid_test - Needs lots of work.

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
