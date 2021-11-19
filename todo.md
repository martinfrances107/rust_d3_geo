TODO:-

* implement path/centroid_test.rs

* Draw outline om graticule test.

* Debug rendering of land from 50m.json

* Check on failing area test in view of recent major bug fix. ( Still broken )

* Performance is not showing the expected gains over javascript ..
	I am not caching projection::stream()
	I think there is lots of copying of data going on where pass by reference is needed this needs review.
	I am using the default HashMap .. which the docs say by default using a secure version which always completes
	in a resonable time ( prvents DOS attacks).. but is also unperformant.

* Refactor trait names ending in Trait.

* Enum names ending in Enum


# Short Term Goals

* Test ClipRectangle.
* Justify whey PathResult is not implemented for LengthStream? is cal() a deviation from the interface.
* Same arguement for CicleStream.

# Not Yet Implemented

* src/contains.js
* src/bounds.js
* src/centroid.js
* sr/interpolate.js
* src/graticule.js
* src/path/area.js
* src/projection/conic.js
* src/projection/albers.js
* src/projection/albersUsa.js
* src/projection/natualEarth.js

# The State of Testing

src/rotation

area_test - Some tests not yet implemented. Exposes bug, also not complete.

bounds_test - src or test not yet implemented.

centroid_test - Needs lots of work.

clip_circle_test - Needs regex development.. Path::API has changed since this last compiled.

contains_test - sec and test not implmented.

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
