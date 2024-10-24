import {
  geoAlbers,
  geoGraticule10,
  geoPath,
  GeoPermissibleObjects,
} from "d3-geo";

export default async function albers(world: GeoPermissibleObjects) {
  const canvas: HTMLCanvasElement | null = document.querySelector("#albers-js");

  if (canvas === null) {
    return;
  }
  const context = canvas.getContext("2d");

  if (context == null) {
    return;
  }
  const width = canvas.width;
  const height = canvas.height;

  const projection = geoAlbers()
    .scale(width)
    .translate([width / 2, height / 2]);

  const path = geoPath(projection, context);

  context.strokeStyle = "#69b2a2";
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();
}
