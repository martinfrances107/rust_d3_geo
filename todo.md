# TODO

-[] - MultiThreaded

  6 Stage pipeline

    /// `StreamTransformRadians` -> `StreamTransform` -> `Preclip` -> `Resample` -> `Postclip` -> `DRAIN`

- [x] StreamTransformRadians
- [x] StreamTransform
- [] PreClip buffers rectangle lineA lineC
- [x] Resample
- [] Postclip Identity/

19 EndPoints need conversion.

Must bundle into a SUPER large Enum  SUPER == suspicious

  todo!() need to list.
  Area, Centroid,

- [x] - Dcoument when the re-enter trait is to be used.

- [x] - Can I precompute values in compose?

[]- What is new in cargo tarpaulin
  albersUSA implement cache in 'fn stream()'
  copy documentation of parallels.

High Priority.
examples/projection eslint needs updating.

- Write examples/index.html -- a guide to building rust projects.

- Develop styling for world maps graticules on bottom, overlaid by land.

- Add examples landing page. - showing images of the each projection.

- Review  path/centroid_test.rs
    has a small number of low priority test to implement.

- Examples projections CSS styling needs minor adjustment font sizes of text
   looks off ..select/apply a simple responsive css template.
  ( eg. Boilerplate, material-ui )

## WebGl Demo

 [] Render points to fast.html
    clip space issue?

 [] Render polygons
    data format Cx, Cy, x0, y0, x1, y1 ...
    where Cx, Cy are the voronoi centers
    x0, y0, x1, y1 are the points of the delaynay cell.

  [] rotation in a loop.

## Before API lock-down

- Detail why Feature and FeatureCollection is complicated.

- cargo docs:
  Some traits are missing description.

- Define graticule10 as an alias for Graticule::default().
 -- the code in examples looks scrappy.

- Justify whey PathResult is not implemented for LengthStream? is cal() a deviation from the interface.

- Same argument for CircleStream.

## Not Yet Implemented

- src/interpolate.js

## The State of Testing

snapshot tests are missing.

src/rotation ?

area_test - Some tests not yet implemented. Exposes bug, also not complete.

bounds_test - src or test not yet implemented.

centroid_test - Needs minor additions

clip_circle_test - Needs regex development.. Path::API has changed since this last compiled.

contains_test - src and test not implemented.

distance_test - Complete.

equirectangular_test - Complete.

invert_test - Complete.

index_test - Need to get mocks working.

length_test -  Needs feature collection tests.

mercator_test - Bugs lurking .. see commented out tests.

projection_angle_test  - Needs geoIdentity test

path_area_test - Complete.

path_bounds_test - Complete.

path_string_test - Need to port test for Features, FeatureCollection, GeometryCollection, Polygon and LineString.

polygon_contains_test - Complete.

projection_reflect_test - Complete.

stereographic_test - Complete.

snapshot test - Must implement from scratch.
