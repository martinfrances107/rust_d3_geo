
# Rust D3 Geo

Rust 2021 Edition.
<div align="center">

<a href="https://crates.io/crates/d3_geo_rs"><img alt="crates.io" src="https://img.shields.io/crates/v/d3_geo_rs.svg"/></a>
<a href="https://docs.rs/d3_geo_rs" rel="nofollow noopener noreferrer"><img src="https://docs.rs/d3_geo_rs/badge.svg" alt="Documentation"></a>
<a href="https://crates.io/crates/d3_geo_rs"><img src="https://img.shields.io/crates/d/d3_geo_rs.svg" alt="Download" /></a>

</div>

This is a port [d3-geo](https://github.com/d3/d3-geo) into RUST.

A collection of d3 sub packages is being ported to rust.

* d3_geo_rs
* [d3_delaunay_rs](https://crates.io/crates/d3_delaunay_rs)
* [d3_geo_voronoi_rs](https://crates.io/crates/d3_geo_voronoi_rs)

## Current Status

The majority of the library has been ported along with the associated tests.

The 1.0 release is close, within one week.

Currently I am reviewing the data structures, and looking for performance improvements before things become locked down and I support backwards compatibility guarentees.

## When to use the rust version of the library

The limits of the javascript library become obvious when developing interactive applications that process large datasets.
For example the examples/globe applications operate on a 1:50M resolution map of the earth. On a desktop machine this is beyond the javascript version.

<table>
<th align="left" colspan="3">Supported Projections</th>
<tr><td>Albers</td><td>Equidistant</td><td>Mercator</td></tr>
<tr><td>AlbersUsa</td><td>Equirectangular</td><td>MercatorTansverse</td></tr>
<tr><td>Aziumuthal Equal Area</td><td>Equal Area</td><td>Orthographic</td></tr>
<tr><td>Azimuthal EquiDistant</td><td>Equal Earth</td><td>Stereographic</td></tr>
<tr><td>Conformal</td><td>Gnomic</td><td></td></tr>
</table>

## Examples

These Examples are provided to help developers convert their existing javascript to rust. They can be found in the github reposository associated with this crate.

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

For perfomance reasons this example is best viewed by running cargo build and then "cargo serve" which compiles the rust code using the --release flag.

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

 SVG are useful  when the semantic meaning of the data needs to be preserved. The example shows how to load/parse/display the globe as indivdual SVG PATH elements.

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
The typescript is responsible for handling the mouse events and calculating the quaternion and finally calculating the appropiate change in rotation. In a typescript render loop calls to a rust function render the globe.(Scale 1:50M)
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

This globe is rendered to a HTML CANVAS element

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

The globe is rendered as a SVG image.

Show all the counties in the USA.

AlbersUSA is unlike the other projections. Alaska and Hawaii are rendered as insets.
As can be see in the code a Multidrain must be used to gather the three projections.

(Scale of 1:10M)

</td>
<td><image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/albers_usa.svg"></td>

</tr>
<tr>

<td>
<strong>examples/ring</strong><br/><br/>
SVG example


Sample code in both RUST and javascript that renders a complex multipolygon. ( Orthographic and Sterographic )

</td>
<td>
<image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/ring.png">
</td>

</tr>
</tbody>
<table>

<br/>

Here is an outline of the common steps found in all the examples.

1) Take a projection's default builder, make adjustments, then call build() to construct a projector.

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

    A Path is a collection of nodes where each step on the path transforms the geometry object.

    A variey of endpoint are available Area, Length, Centroid, but these examples deal
    only with rendering to a HTML canvas element or a SVG path element.

    When rendering to a HTML canvas element build path from a Path2D "rendering conext"

      ```rust
       //  Construct a PathBuilder linked to Path2d
       // rendering context.
       let path2d = Path2d::new()?;
       let endpoint = PathBuilder::new(path2d);
       let pb = PathBuilder::new(endpoint);
       let path = pb.build();
      ```

    Generating a SVG image

      ```rust
        let pb = PathBuilder::pathstring();
        let path = pb.build();
      ```

3) Please see the different examples, but the common next step is to
   construct a PathBuilder object and then to stream a geometry object into it :-

      ```rust
         // 'countries' is a geometry extratced from
         // a world map json file.
         path.stream(&countries)
      ```

## Running the examples

<br/>
Requirements:

* node and npm [installation guide](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

* wasm-pack [installation guide](https://rustwasm.github.io/wasm-pack/installer/)

<br/>

To view the application either create a devleopment build, or construct a static-web site as follows

### Start And Run A Development Build

 ```console
 git clone https://github.com/martinfrances107/rust_d3_geo.git
 cd rust_d3_geo/examples/ring/
 npm install
 npm run start
 ```

The last command "npm run start"  will automatically open your default browser at http:://localhost::8080

### Performance: Building A Static Site

Much better performance can be acheived by bulding a static web site and viewing that directly. At the expense of longer build times the RUST protions of the code a build using the "--release" tags

```console
  git clone https://github.com/martinfrances107/rust_d3_geo.git
  cd rust_d3_geo/examples/ring
  npm install
  npm run build
  npm run serve
```

</details>
<br>

## Benchmarking

<details>
<summary>See the details.</summary>
In this project, we have two benchmarks, based on the ring and graticule examples ( see above. )

Also [rust_d3_geo_voronoi](https://github.com/martinfrances107/rust_d3_geo_voronoi)
 uses this library, and that project contains a benchmark which contains an exact port of a benchmark in [d3-geo-voronoi](https://github.com/Fil/d3-geo-voronoi).
 Based on that benchmark rust is 31% faster, or permits a 37% increase in throughput.
</details>

<br>

## Flamegraph

profile_target is binary that outputs a HTML page containing a SVG image showing the globe with graticule markings.

A flamegraph can be created with the following

```bash
cd profile_target
sudo CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph
```

The complexity of rendering 240 countries/polygons provides a good view in memory allocation issues.

<br>

## Coding Standard

* Idomatic RUST, as defined by cargo clippy where possible.
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

   This allow for a clearer statement of intent :-

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

Version 1.0 is about to become public, where major changes are discourged.

* [rayon](https://docs.rs/rayon/latest/rayon/index.html) is rust's crate for multithread support.
I have made extensive use of iterators when porting the code and rayon support the easy conversion of single threaded iterators to multithread iterators.

* The Hashmaps - appear slow.
  Maybe I can get performace improvements by replacing them with B-tree collections?

<br>

#### Architecture discussion

There is an aspect of the design that needs review. It related to the best way to implement a doubly-linked list which has cross links between nodes.

The clipping algorithm in clip/rejoin/mod.rs needs to be refactored.
see  [The intersection Problem.](/intersection_problem.md)
Test coverage in that area is high so the algortihms is working but the data structures make extensive use of vectors ( heap objects ) containng references to other heap objects ```Vec<Options<Rc<RefCell<_Intersection_>>>>```   which is not performant.

 A full discusion can be found [here](/intersection_problem.md)

### Unimplemented sections of the library

Support for a custom projection is not yet supported.
For an example of this see the test labelled "projection.fitExtent(…) custom projection"
