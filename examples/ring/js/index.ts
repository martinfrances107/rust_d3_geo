import { GeoProjection } from "d3";
import { MultiPolygon } from "geojson";
import { geoCircle, geoOrthographic, geoPath, geoStereographic } from "d3-geo";

async function drawStereographic() {
  console.log("draw()");
  const svg: SVGSVGElement = document.getElementsByTagName("svg")[3];

  const width = svg.clientHeight;
  const height = svg.clientWidth;
  svg.querySelector("path")?.remove();

  const ortho: GeoProjection = geoStereographic()
    // .scale(240)
    .translate([width / 2, height / 2])
    .rotate([0, 0, 0]);

  const cg_outer = geoCircle().radius(10).precision(10);
  const cg_inner = geoCircle().radius(5).precision(5);
  const coordinates = [];

  for (let lat = -30; lat <= 30; lat += 30) {
    for (let long = -180; long < 180; long += 40) {
      const poly = [
        cg_outer.center([long, lat])().coordinates[0],
        cg_inner.center([long, lat])().coordinates[0].reverse(),
      ];

      coordinates.push(poly);
    }
  }

  const object: MultiPolygon = {
    coordinates,
    type: "MultiPolygon",
  };

  const d = geoPath().projection(ortho)(object);

  if (d !== null) {
    const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
    path.setAttributeNS(null, "d", d);
    svg.appendChild(path);
  }
}

async function drawOrthographic() {
  console.log("draw()");
  const svg: SVGSVGElement = document.getElementsByTagName("svg")[1];

  const width = svg.clientHeight;
  const height = svg.clientWidth;
  svg.querySelector("path")?.remove();

  const ortho: GeoProjection = geoOrthographic()
    .translate([width / 2, height / 2])
    .rotate([0, 0, 0]);

  const cg_outer = geoCircle().radius(10).precision(10);
  const cg_inner = geoCircle().radius(5).precision(5);
  const coordinates = [];

  for (let lat = -30; lat <= 30; lat += 30) {
    for (let long = -180; long < 180; long += 40) {
      const poly = [
        cg_outer.center([long, lat])().coordinates[0],
        cg_inner.center([long, lat])().coordinates[0].reverse(),
      ];

      coordinates.push(poly);
    }
  }

  const object: MultiPolygon = {
    coordinates,
    type: "MultiPolygon",
  };

  const d = geoPath().projection(ortho)(object);
  if (d !== null) {
    const path = document.createElementNS("http://www.w3.org/2000/svg", "path");

    path.setAttributeNS(null, "d", d);
    svg.appendChild(path);
  }
}

// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import("../pkg");

rust
  .then((m) => {
    console.log("wasm is imported");
    m.run();
  })
  .catch(console.error);

Promise.all([drawOrthographic(), drawStereographic()]);
window.addEventListener("resize", async () => {
  Promise.all([drawOrthographic(), drawStereographic()]);
});
