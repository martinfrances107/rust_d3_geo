# Rust D3 Geo
Rust 2021 Edition.

This is a port  [d3-geo](https://github.com/d3/d3-geo) into RUST.
I am actively working towards a [Alpha release](https://github.com/martinfrances107/rust_d3_geo/milestone/1) milestone. I intended to eventually publish this on [crates.io](https://crates.io/)

## Examples
Examples are provided to help developers convert their existing javascript to rust.
To run the example please follow the "Running the examples" below.

<table>
<thead>
<tr>
<th align="left">Description</th>
<th align="right"></th>
</tr>
</thead>
<tbody align="left" style="vertical-align:top;">

<tr>
<td><strong>examples/globe/canvas_rotating_50m</strong> <br/><br/>  This is a port into rust of this d3-geo example <a href="https://www.d3indepth.com/geographic/">www.d3indepth.com/geographic/</a>.  <br/><br/> For perfomance reasons this example is best viewed as a static web site, rather the running the development build.
<br/>
<br/>
</td>
<td><image src="images/rotating.png"></td>
</tr>
<tr>
<tr>
<td><strong>examples/globe/svg</strong> <br/><br/> SVG are useful  when the semantic meaning of the data needs to be preserved. The example shows how to load/parse/display the globe as indivdual SVG PATH elements. It also includes code samples that generates SVG graticules. </td>
<td><image src="images/globe.svg"> </td>
</tr>
<td> <br/><br/>  
  This is similar to the other globe applications. As an example it deliberatly mixes typescript methods with rust.
  The typescript is responsible for handling the mouse events and calculating the euler angle, forming the appropiate quaternion and finally calcuting the appropiate change in rotation. RUST then takes the rotation and redraws the canvas.
  
<br/>
<br/>
</td>
<td></td>
</tr>

<tr>
<td><strong>examples/projections</strong> <br/><br/>

As a confidence building exercise, this demo
shows a side by side comparison of the all the projections rendered by in both  <strong>javascript</strong> and <strong>rust</strong>. </td>
<td><image src="images/projection.png"> </td>
</tr>
<tr>
<td> <strong>examples/ring</strong><br/>Renders a complex multipolygon. </td>
<td><image src="images/ring.png"></td>
</tr>
</tbody>
<table>


## When to use the rust version of the library

The limits of the javascript library become obvious when developing interactive applications that process large datasets.
For example the examples/globe applications operate on a 50m resolution map of the earth. On a desktop machine this is beyond the javascript version.

## Current Status

The majority of the library has been ported along with the associated tests. The aim is to eventaully release this as a rust crate.
but at the moment is this alpha grade software.

<br/>
<details>
<summary> Here is a summary of things to-do before the crate is published.</summary>

* The recenter state-based API refacor is almost complete.
  Once fit_size_resampling() is reinstated the code-coverage metric will jump 10% back to the previous value of approx 82%

* The API is not stabalised. If perfomance issues arise then the API will change. Additionaly I plan a final review to remove anything uneeded before making changes become complicated.

* The clipping algorithm in clip/rejoin/mod.rs needs to be refactored.
see  [The intersection Problem.](#the-intersection-problem--request-for-comments)
Test coverage in that area is high so the algortihms is working but the data structures make extensive use of vectors ( heap objects ) containng references to other heap objects ```Vec<Options<Rc<RefCell<_Intersection_>>>> ```   which is not performant.

</details>

## Running the examples

<details>
<summary>See the details.</summary>

<br/>
Requirements:

 * node and npm [installation guide](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

 * wasm-pack [installation guide](https://rustwasm.github.io/wasm-pack/installer/)

<br/>

To view the application either create a devleopment build, or construct a static-web site as follows

 ### Start And Run A Development Build
 ```console
 cd rust_d3_geo/examples/ring/
 cargo build
 npm install
 npm start
 ```

The last command "npm run start"  will automatically open your default browser at http:://localhost::8080

### Building A Static Site

Much better performance can be acheived by bulding a static web site and viewing that direclty.

```console
  cd rust_d3_geo/examples/ring
  cargo build
  npm run  build
```

This creates a static HTML site in the dist/ directory.

To view the site you cannot just point you browser at a location of the form file:://home/user/alice/dist/index.html

By security reasons, browsers prevent HTML pages with WASM files from being viewed this way. You must host the site first.

A way forward here is to use a npm package called serve

```console
  sudo npm install --global serve
  serve dist
```

If everything works you will be given a locaation to view

For example http:://localhost::3000

</details>
<br>

## Benchmarking

<details>
<summary>See the details.</summary>
In this project, we have two benchmarks, based on the ring and graticule examples ( see above. )

Also [rust_d3_geo_voronoi](https://github.com/martinfrances107/rust_d3_geo_voronoi)
 uses this library, and that project contains a benchmark which contains an exact port of a benchmark in [d3-geo-voronoi ](https://github.com/Fil/d3-geo-voronoi).
 Based on that benchmark rust is 31% faster, or permits a 37% increase in throughput.
</details>

<br>

## Future Multi thread support
<details>
<summary>See the details.</summary>
On my todo list.

* [rayon](https://docs.rs/rayon/latest/rayon/index.html) is rust's crate for multithread support.
I have made extensive use of iterators when porting the code and rayon support the easy conversion of single threaded iterators to multithread iterators.

* The Hashmaps - appear slow.
  Maybe I can get performace improvements by replacing them with B-tree collections?
</details>

<br>

## Architecture discussion

There is an aspect of the design that needs review. It related to the best way to implement a doubly-linked list which has cross links between nodes. A full discusion can be found [here](/intersection_problem.md)

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
       fn reflect_x_set(&mut self, reflect: REFLECT) -> &mut Self;
      /// Set the projection builder to invert the y-coordinate.
       fn reflect_y_set(&mut self, reflect: REFLECT) -> &mut Self;
   }
   ```

   This allow for a clearer statement of intent :-
   ```rust
   builder.reflect_y_set(REFLECT::Flipped);
   ```
 * ["Type-Driven API Design"](https://www.youtube.com/watch?v=bnnacleqg6k) is the
     preferred way of constructing state machines.

     In the example below, when assembling a stream pipeline, connect() can only be called
     when the state is "Unconnected". The output type's STATE is "Connected<SINK>".

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
     The "Stream" trait is only implemented when the STATE is "Connected<SINK>".
     By design, all code is prevented from calling line_start() or point() unless the object
     has been connected to another pipeline stage.

## Unimplemented sections of the library.

<details>
<summary>See the details.</summary>
Support for a custom projection is not yet supported.
For an example of this see the test labelled "projection.fitExtent(…) custom projection"

I am trying to get a program of mine to run faster, but I want this to eventually be a true library port. So feel free to add suggestions to my todo list.

Here is a list of the currently supported projections.
* AziumuthalEqualArea
* AzimuthalEquiDistant
* Equirectangular
* Gnomic
* Mercator
* Orthographic
* Stereographic

A complete list of all ported projections can be found in invert-test.rs. Out of the 15 distinct projections listed only 7 have been ported so far.
</details>

<br>

## Other To-do's

<details>
<summary>See the details.</summary>

## Document API changes such as
  * src/projection/clip_angle_reset()
  * src/projection/clip_extent_clear()

Finally

[todo.md](/todo.md) contains a more detailed list
</details>
