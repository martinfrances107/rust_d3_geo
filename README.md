
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

This library allows the development of custom maps. It provides a comprehensive set
 of projections along with the means to scale rotate and translate the final image.
 The projector processes polygons, lines and points in the form of
 [GeoJSON](https://en.wikipedia.org/wiki/GeoJSON) objects. Additionally this
 library can be used to calculate lengths, areas and the centroid of such objects

[CHANGELOG.md](https://github.com/martinfrances107/rust_d3_geo/blob/v2.x-dev/CHANGELOG.md) contains a summary of breaking changes between v1.x, 2.x and 3.x

Version 3.0.0 adds support for WGPU

> "WGPU is a pure-rust graphics API. It runs natively on Vulkan, Metal, D3D12, and OpenGL; and on top of WebGL2 and WebGPU on wasm." [\[wgpu\]](https://crates.io/crates/wgpu)

## When to use the rust version of the library

The limits of the javascript library become obvious when developing interactive applications that process large datasets.
For example the examples/globe applications operate on a highly detailed ( 1:50M resolution ) map of the earth. On a laptop this is beyond the javascript version.

 | Available Projections |   |   |
 | --------------------- | - | - |
 | Albers | ConicEqualArea | Gnomic|
 | AlbersUsa | Equidistant | Orthographic |
 | AzimuthalEqualArea | Equirectangular | Mercator |
 | AzimuthalEquiDistant | EqualArea | MercatorTransverse |
 | Conformal | EqualEarth | Stereographic |

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

The javascript version compromises by using a low resolution map. Here no such compromise is required.

This globe is rendered to a HTML CANVAS element.

For performance reasons this example is best viewed by running "npm run  build" and then "npm run serve" which compiles the rust code using the --release flag.

(Scale 1:50M)

</td>

<td width="50%">
  <image src="https://raw.githubusercontent.com/martinfrances107/rust_d3_geo/main/images/rotating.png">
</td>

</tr>
<tr>
<td width="50%">
<strong>examples/globe/rotating_WGPU</strong><br><br>

[WGPU](https://gpuweb.github.io/gpuweb/) support in the browser is partial and currently hidden behind experimental flags. See the browser-WGPU Implementation [Status](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status). For now development in this library uses the [winit](https://crates.io/crates/winit) crate to make cross platform application.

The promise of this approach is to bypass the bottlekneck in passing bulk data from RUST memory space, into javascript, and finally into GPU memory.

GeoJson Geometry is streamed through this libraries rendering pipeline into a new **PolyLinesWPGU** endpoint.

This endpoint's output is a (vertex_buffer,index_buffer) pair in the form of blocks of contiguous memory which can be passed directly to the GPU.

A thin vertex and fragment shader is then responsible for rendering.

The example requires the feature flag "wgpu".

(Scale 1:50M)

</td>

<td width="50%">
<image src="https://github.com/martinfrances107/rust_d3_geo/blob/v2.x-dev/images/single.png">
</td>

</tr>
<tr>

<td>
<strong>examples/globe/svg</strong><br><br>

The globe is rendered as a SVG image.

SVG are useful when the semantic meaning of the data needs to be preserved. The example shows how to load/parse/display the globe as individual SVG PATH elements.

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
shows a side by side comparison of the all the projections rendered by in both <strong>javascript</strong> and <strong>rust</strong>.

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

### An outline of the common steps found in all the examples

1) For a given projection, use its default projection builder, make changes to the scale, translation .. etc, then call build() to construct a projector.

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

     **Clipping**: Two strategies are used "Antimeridian" and "ClipAngle" [See clip_angle_set() and clip_angle_reset()]

     **Resampling**: Dense geometry can be reduced by declaring a separation distance under which points, used to describe polygons and lines, are considered indistinguishable [See precision_set()]

     **Bounding**: A projection space box can be set, and only geometry within this extent will be displayed. Polygons partially inside the box are restructured to conform to the edges of the box. [See clip_extent_set() clip_extent_clear()]

      **Endpoints** are special path nodes which hold the result of a calculation. A variety of endpoint are available Area, Centroid, Length which can be use to compute properties about polygons or lines. These examples only show endpoints that render to a HTML canvas element or a SVG path element.

    When rendering to a HTML canvas the endpoint holds Path2D "rendering context"

      ```rust
       // Construct a PathBuilder linked to Path2d
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

### Requirements (running the examples)

* node and npm [installation guide](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

* wasm-pack [installation guide](https://rustwasm.github.io/wasm-pack/installer/)

<br/>

To view an example application either create a development build, or construct a static-web site as follows

### Examples: Start And Run A Development Build

 ```console
 git clone https://github.com/martinfrances107/rust_d3_geo.git
 cd rust_d3_geo/examples/ring/
 npm install
 npm run start
 ```

The last command "npm run start" will automatically open your default browser at http:://localhost::8080

### Performance: Examples Building A Static Site

Much better performance can be achieved by building a static web site and viewing that directly. At the expense of longer build times the RUST portions of the code a build using the "--release" tags

```console
  git clone https://github.com/martinfrances107/rust_d3_geo.git
  cd rust_d3_geo/examples/ring
  npm install
  npm run build
  npm run serve
```

## Feature List

* **web** (default) This feature allows rendering to a HTML CANVAS element. For CLI binary applications this can be removed to reduce the dependency count.

* **wgpu** Allow an endpoint that outputs points and poly-lines as "Array Buffer". This buffer can then be fed direcly in a the GPU reducing the amount system calls. This feature is highly experimental.

## Benchmarking

There are two distinct compute environments

  1. The browser: Is a highly constrained environment.
  Here are two equivalent benchmarks, one for rust
  [rust_d3_geo_voronoi](https://github.com/martinfrances107/rust_d3_geo_voronoi)
  and one for javascript [d3-geo-voronoi](https://github.com/Fil/d3-geo-voronoi).
  Rust runs as a service worker with no direct control the DOM. Passing objects as JsValue
  between threads has negative performance impacts despite this benchmark runs **twice** as fast.

  2. Node like environments:
  The github repository associated with crate has two "profile targets" and two "benches"
  which can be used to to spot bottlenecks in this environment. The benches are [Criterion.rs](https://crates.io/crates/criterion) based micro benchmarks.

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

"Semantic Versioning" guidelines :-

* Increment the major number when a breaking change occurs.
* Increment the minor number when a new feature is added, @deprecated notes added to outdated functions,
* Increment the patch number for tightly focused security fixes.

Future Work.

* Since I ported this code ... the javascript has added a digits() function to control the
  precision of numbers as they are output as strings. ( I need to implement this upgrade )

* [rayon](https://docs.rs/rayon/latest/rayon/index.html) is rust's crate for multithread support. Since this crate is focused on web development and so incoperating [wasm-bindgen-rayon](https://crates.io/crates/wasm-bindgen-rayon). wasm-bindgen_rayon recommends using nightly - and so it far from production read. I have made extensive use of iterators when porting the code and rayon support the easy conversion of single threaded iterators to multithread iterators.

## Architecture discussion

There is an aspect of the design that needs review. It related to the best way to implement a doubly-linked list which has cross links between nodes.

The clipping algorithm in clip/rejoin/mod.rs needs to be refactored.
see [The intersection Problem.](/intersection_problem.md)
Test coverage in that area is high so the algorithms is working but the data structures make extensive use of vectors ( heap objects ) containing references to other heap objects ```Vec<Options<Rc<RefCell<_Intersection_>>>>``` which is not performant.

 A full discussion can be found [here](/intersection_problem.md)

### Unimplemented sections of the library

Support for a custom projection is not yet supported.

For an example of this see the test labelled "projection.fitExtent(…) custom projection"
