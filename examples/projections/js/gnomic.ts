import {
  geoGnomonic,
  geoGraticule10,
  geoPath,
  GeoPermissibleObjects,
} from "d3-geo";

export default async function gnomic(world: GeoPermissibleObjects) {
  const canvas: HTMLCanvasElement = document.querySelector("#gnomic-js");

  const context = canvas.getContext("2d");

  const width = canvas.width;
  const height = canvas.height;

  const projection = geoGnomonic()
    .scale(width / 6)
    .translate([width / 2, height / 2])
    .precision(0.3)
    .clipAngle(90 - 1e-3);

  const path = geoPath(projection, context);

  context.strokeStyle = "#69b2a2";
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();
}
