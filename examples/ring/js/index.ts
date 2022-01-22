// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('../pkg');

rust
  .then(m => {
    console.log("wasm is imported");
    m.run()
  })
  .catch(console.error);

import { GeoProjection } from "d3";
import { MultiPolygon } from 'geojson';
import {
  geoCircle,
  geoOrthographic,
  geoPath
} from "d3-geo";

function draw() {
  console.log("draw()");
  var svg: SVGSVGElement = document.getElementsByTagName("svg")[1];

  const width = svg.clientHeight;
  const height = svg.clientWidth;
  svg.querySelector("path")?.remove();

  const ortho: GeoProjection = geoOrthographic()
    .scale(240)
    .translate([width / 2, height / 2])
    .rotate([0, 0, 0]);

  let cg_outer = geoCircle().radius(10).precision(10);
  let cg_inner = geoCircle().radius(5).precision(5);

  let coordinates = [];

  for (let lat = -30;lat <= 30;lat += 30) {
    for (let long = -180;long < 180;long += 40) {
      const poly = [cg_outer.center([long, lat])().coordinates[0],
      cg_inner.center([long, lat])().coordinates[0].reverse()];

      coordinates.push(poly);
    }
  }

  let object: MultiPolygon = {
    type: "MultiPolygon",
    coordinates: coordinates
  };

  let d = geoPath().projection(ortho)(object);


  var path: SVGPathElement = document.createElementNS('http://www.w3.org/2000/svg', 'path');
  path.setAttributeNS(null, "d", d);

  const a = svg.appendChild(path);

}

draw();
window.addEventListener('resize', (event) => {
  draw()
}

);
