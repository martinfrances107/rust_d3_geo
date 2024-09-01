import {
  geoGraticule10,
  geoMercator,
  geoPath,
  GeoPermissibleObjects,
} from "d3-geo";

export default async function mercator(world: GeoPermissibleObjects) {
  const canvas: HTMLCanvasElement | null =
    document.querySelector("#mercator-js");

  if (canvas == null) {
    return;
  }

  const context = canvas.getContext("2d");

  if (context == null) {
    return;
  }

  const width = canvas.width;
  const height = canvas.height;

  const projection = geoMercator()
    .scale(width / 1.3 / 3.14)
    .translate([width / 2, height / 2])
    .clipAngle(90)
    .precision(0.1);

  const path = geoPath(projection, context);

  context.strokeStyle = "#69b2a2";
  context.stroke();
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();
}
