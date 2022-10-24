# Rust D3 Geo
Rust 2021 Edition.

This is a port  [d3-geo](https://github.com/d3/d3-geo) into RUST.

## When to use the rust version of the library

The limits of the javascript library become obvious when developing interactive applications that process large datasets.
For example the examples/globe applications operate on a 50m resolution map of the earth. On a desktop machine this is beyond the javascript version.

## Status
I am acitvely maintaining a [Alpha release](https://github.com/martinfrances107/rust_d3_geo/milestone/1) milestone.

The majority of the library has been ported along with the associated tests. The aim is to eventaully release this as a rust crate.
but at the moment is this alpha grade software. 

<details>
<summary> Here is a summary of things to-do before the crate is published.</summary>

* The recenter state-based API refacor is almost complete.
  Once fit_size_resampling() is reinstated the code-coverage metric will jump 10% back to the previous value of approx 82%

* The API is not stabalised. If perfomance issues arise then the API will change. Additionaly I plan a final review to remove anything uneeded before making changes become complicated.

* The clipping algorithm in clip/rejoin/mod.rs needs to be refactored.
see  [The intersection Problem.](#the-intersection-problem--request-for-comments)
Test coverage in that area is high so the algortihms is working but the data structures make extensive use of vectors ( heap objects ) containng references to other heap objects ```Vec<Options<Rc<RefCell<_Intersection_>>>> ```   which is not performant.

</details>

<br>

## Examples

Examples are provided to help developers convert their existing javascript to rust.
To run the example please follow the "Running the examples" below. The globe examples are interactve and for perfomance reasons they are best viewed as a static web site.

| Name                               | Description                                                                                                                                                                                                                                  |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| examples/globe/canvas              | Shows how to load/parse/display  a complex topojson file to a CANVAS element.                                                                                                                                                                |
| examples/globe/canvas_rotating_50m | **At the moment this example is very experimental** This is a conversion of the javascript d3_geo example found in https://www.d3indepth.com/geographic/ It uses the gloo_timer module to emulate calls to window.setInterval(update, 100);. |
| examples/globe/svg                 | SVG are useful  when the semantic meaing of the data needs to be preserved. The example shows how to load/parse/display the globe as indivdual SVG PATH elements. It also includes code samples that generates SVG graticules.               |
| examples/projections               | Shows a side by side comparison of the all the projections rendered by both **javascript** and rust.                                                                                                                                         |
| examples/ring                      | Renders a complex multipolygon.                                                                                                                                                                                                              |
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
 uses this library, and thank project contains a benchmark which contains an exact port of a benchmark in [d3-geo-voronoi ](https://github.com/Fil/d3-geo-voronoi).
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
