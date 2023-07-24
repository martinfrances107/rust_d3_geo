
# Rust D3 Geo

Rust 2021 Edition.
<div align="center">

<a href="https://crates.io/crates/d3_geo_rs"><img alt="crates.io" src="https://img.shields.io/crates/v/d3_geo_rs.svg"/></a>
<a href="https://docs.rs/d3_geo_rs" rel="nofollow noopener noreferrer"><img src="https://docs.rs/d3_geo_rs/badge.svg" alt="Documentation"></a>
<a href="https://crates.io/crates/d3_geo_rs"><img src="https://img.shields.io/crates/d/d3_geo_rs.svg" alt="Download" /></a>

</div>

This is a port [d3-geo](https://github.com/d3/d3-geo) into RUST. It is part of a family of ported d3-modules

* d3_geo_rs
* [d3_delaunay_rs](https://crates.io/crates/d3_delaunay_rs)
* [d3_geo_voronoi_rs](https://crates.io/crates/d3_geo_voronoi_rs)

The [GeoJSON](https://en.wikipedia.org/wiki/GeoJSON) format can be used to define polygons, line and points. This library can be used to calculate  lengths, areas and the centroid of such objects. Additionally a comprehensive set of projections is provided along with a to means to manipulate the scaling, translation and rotation - allowing the user to develop custom maps.

[CHANGELOG.md](https://github.com/martinfrances107/rust_d3_geo/blob/v2.x-dev/CHANGELOG.md) contains a summary of breaking changes between v1.x and 2.0.0.

## When to use the rust version of the library

The limits of the javascript library become obvious when developing interactive applications that process large datasets.
For example the examples/globe applications operate on a highly detailed ( 1:50M resolution )  map of the earth. On a desktop machine this is beyond the javascript version.

<table>
<th align="left" colspan="3">Supported Projections</th>
<tr><td>Albers</td><td>Equidistant</td><td>Mercator</td></tr>
<tr><td>AlbersUsa</td><td>Equirectangular</td><td>MercatorTransverse</td></tr>
<tr><td>Azimuthal Equal Area</td><td>Equal Area</td><td>Orthographic</td></tr>
<tr><td>Azimuthal EquiDistant</td><td>Equal Earth</td><td>Stereographic</td></tr>
<tr><td>Conformal</td><td>Gnomic</td><td></td></tr>
</table>

## Examples

These Examples are provided to help developers convert their existing javascript to rust. They can be found in the github repository associated with this crate.

<table>
<thead>
<tr>
<th align="left" colspan="2" >Description</th>
</tr>
</thead>
<tbody align="left" style="vertical-align:top;">
<tr>
<td width="50%">
<strong>examples/globe/rotating</strong><br><br>

This is a port into rust of this d3-geo example

[www.d3indepth.com/geographic/](https://www.d3indepth.com/geographic/)

This globe is rendered to a HTML CANVAS element.

For performance reasons this example is best viewed by running "cargo build" and then "cargo serve" which compiles the rust code using the --release flag.

(Scale 1:50M)

</td>

<td width="50%">
  <image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/rotating.png">
</td>

</tr>
<tr>

<td>
<strong>examples/globe/svg</strong><br><br>

The globe is rendered as a SVG image.

 SVG are useful  when the semantic meaning of the data needs to be preserved. The example shows how to load/parse/display the globe as individual SVG PATH elements.

  It also includes code samples that generates SVG graticules.

(Scale 1:50M)
</td>
<td>
<image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/globe.svg">
</td>

</tr>
<tr>

<td><strong>examples/globe/drag_and_zoom</strong> <br/><br/>

This globe is rendered to a HTML CANVAS element

It deliberately mixes typescript methods with rust.
The typescript is responsible for handling the mouse events and manipulating the quaternion used to calculate the appropriate change in rotation. In a typescript render loop calls to a rust function render the globe.
<br/>
<br/>
  This example is currently undergoing rapid development.

  (Scale 1:50M)
</td>

<td>
<image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/drag_and_zoom.png">
</td>

</tr>
<tr>

<td>
<strong>examples/projections</strong> <br/><br/>

All available projections are rendered to a HTML CANVAS element

As a confidence building exercise, this demo
shows a side by side comparison of the all the projections rendered by in both  <strong>javascript</strong> and <strong>rust</strong>.

(Scale 1:50M)
</td>
<td>
<image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/projection.png">
</td>

</tr>
<tr>

<td><strong>examples/globe/albers_usa</strong> <br><br>

This show all the counties in the USA.

AlbersUSA is unlike the other projections. Alaska and Hawaii are rendered as insets.
As can be seen in the code a Multi-drain must be used to gather the three projections.

(Scale of 1:10M)

</td>
<td><image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/albers_usa.svg"></td>

</tr>
<tr>

<td>
<strong>examples/ring</strong><br/><br/>
SVG example


Sample code in both RUST and javascript that renders a complex multi-polygon. ( Orthographic and Stereographic )

</td>
<td>
<image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/ring.png">
</td>

</tr>
</tbody>
<table>

<br/>

## An outline of the common steps found in all the examples.

1) For a given projection, use its default projection builder , make changes to the scale, translation .. etc, then call build() to construct a projector.

    ```rust
    let projector = Stereographic::<f64>::builder()
      .scale_set(100_f64)
      .translate_set(&Coord {
         x: 300_f64,
         y: 300_f64,
      })
      .clip_angle(90_f64)
      .precision_set(&10_f64)
      .build();
    ````

2) Construct a PathBuilder

    A Path is a collection of nodes where each step on the path transforms the geometry object in some manner.

    An object is then streamed along the path.

    Here is an overview of the key nodes.

     **Clipping**: Is the process of removing hidden geometry. When displaying a globe for example africa and australia will never been visible in the same view. Two strategies are used "Antimeridian" and "ClipAngle" [ See clip_angle_set() and clip_angle_reset() ]

     **Resampling**: Dense geometry can be reduced by declaring a separation distance under which points, used to describe polygons and lines, are considered indistinguishable [ See precision_set() ]

     **Bounding**: A projection space box can be set, and only geometry within this extent will be displayed. Polygons partially inside the box are restructured to conform to the edges of the box. [ See clip_extent_set() clip_extent_clear() ]

      **Endpoints** are special path nodes which hold the result of a calculation. A variety of endpoint are available Area, Centroid, Length which can be use to compute properties about polygons or lines. These examples only show endpoints that render to a HTML canvas element or a SVG path element.

    When rendering to a HTML canvas the endpoint holds Path2D "rendering context"

      ```rust
       //  Construct a PathBuilder linked to Path2d
       // rendering context.
       let path2d = Path2d::new()?;
       let endpoint = PathBuilder::new(path2d);
       let pb = PathBuilder::new(endpoint);
       let path = pb.build();
      ```

    When generating a SVG image, the endpoint holds a string value from which a PATH element can be constructed.

      ```rust
        let pb = PathBuilder::pathstring();
        let path = pb.build();
      ```

3) Please see the different examples, but the common next step is to
   construct a PathBuilder object and then to stream a geometry object into it :-

      ```rust
         // 'countries' is a geometry extracted from
         // a world map json file.
         path.stream(&countries)
      ```

## Running the examples

<br/>
Requirements:

* node and npm [installation guide](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

* wasm-pack [installation guide](https://rustwasm.github.io/wasm-pack/installer/)

<br/>

To view an example application either create a development build, or construct a static-web site as follows

## Start And Run A Development Build

 ```console
 git clone https://github.com/martinfrances107/rust_d3_geo.git
 cd rust_d3_geo/examples/ring/
 npm install
 npm run start
 ```

The last command "npm run start"  will automatically open your default browser at http:://localhost::8080

### Performance: Building A Static Site

Much better performance can be achieved by building a static web site and viewing that directly. At the expense of longer build times the RUST portions of the code a build using the "--release" tags

```console
  git clone https://github.com/martinfrances107/rust_d3_geo.git
  cd rust_d3_geo/examples/ring
  npm install
  npm run build
  npm run serve
```

<br>

## Benchmarking

The github repository associated with crate has two "profile targets" and two "benches"
which can be used to to spot bottlenecks in the code.

The benches are [Criterion.rs](https://crates.io/crates/criterion) based micro benchmarks.

See also [rust_d3_geo_voronoi](https://github.com/martinfrances107/rust_d3_geo_voronoi)
 uses this library, and that project contains a benchmark which contains an exact port of a benchmark in [d3-geo-voronoi](https://github.com/Fil/d3-geo-voronoi).
 Based on that benchmark rust is 31% faster, or permits a 37% increase in throughput.


## Flamegraph

A profile_target is binary that outputs a HTML page containing a SVG image showing the globe with graticule markings.

A flame-graph can be created by entering a particular profile targets directory and running :-

```bash
cd profile_target/albers_usa
sudo CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph
```

The complexity of rendering 240 countries/polygons provides a good view in memory allocation issues.

## Coding Standard

* Idiomatic RUST, as defined by cargo clippy where possible.
* No booleans as arguments to functions/methods, use two state enums instead.

   See "Reflect" as an example.

   ```rust
   pub trait ReflectSet {
       /// f64 or f32.
       type T;
      /// Set the projection builder to invert the x-coordinate.
       fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self;
      /// Set the projection builder to invert the y-coordinate.
       fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self;
   }
   ```

   This allows for a clearer statement of intent :-

   ```rust

   builder.reflect_y_set(Reflect::Flipped);
   ```

* ["Type-Driven API Design"](https://www.youtube.com/watch?v=bnnacleqg6k) is the
     preferred way of constructing state machines.

     In the example below, when assembling a stream pipeline, connect() can only be called
     when the state is "Unconnected". The output type's STATE is "Connected\<SINK\>".

    ```rust
    impl StreamTransformRadians<Unconnected> {
      #[inline]
      /// Connect this node to the next element in the pipeline.
      pub const fn connect<EP, SINK, T>(self, sink: SINK) -> StreamTransformRadians<Connected<SINK>>
      where
        SINK: Clone,
      {
        StreamTransformRadians(Connected { sink })
      }
    }
     ```

     The "Stream" trait is only implemented when the STATE is "Connected\<SINK\>".
     By design, all code is prevented from calling line_start() or point() unless the object
     has been connected to another pipeline stage.

<br/>

## Future 2.0 upgrades

Version 1.0 is stable.

"Semantic Versioning" guidelines :-

* Increment the major number when a breaking change occurs.
* Increment the minor number when a new feature is added, @deprecated notes added to outdated functions,
* Increment the patch number for tightly focused security fixes.

Future Work.

 * [rayon](https://docs.rs/rayon/latest/rayon/index.html) is rust's crate for multithread support.

* I have made extensive use of iterators when porting the code and rayon support the easy conversion of single threaded iterators to multithread iterators.

* The Hashmaps - appear slow.
  Maybe I can get performance improvements by replacing them with B-tree collections?


### Architecture discussion

There is an aspect of the design that needs review. It related to the best way to implement a doubly-linked list which has cross links between nodes.

The clipping algorithm in clip/rejoin/mod.rs needs to be refactored.
see  [The intersection Problem.](/intersection_problem.md)
Test coverage in that area is high so the algorithms is working but the data structures make extensive use of vectors ( heap objects ) containing references to other heap objects ```Vec<Options<Rc<RefCell<_Intersection_>>>>```   which is not performant.

 A full discussion can be found [here](/intersection_problem.md)

### Unimplemented sections of the library

Support for a custom projection is not yet supported.

For an example of this see the test labelled "projection.fitExtent(…) custom projection"
