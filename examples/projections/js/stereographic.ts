import {
  geoGraticule10,
  geoPath,
  GeoPermissibleObjects,
  geoStereographic,
} from "d3-geo";

export default async function sterographic(world: GeoPermissibleObjects) {
  const canvas: HTMLCanvasElement | null =
    document.querySelector("#stereographic-js");

  if (canvas == null) {
    return;
  }

  const context = canvas.getContext("2d");

  if (context == null) {
    return;
  }

  const width = canvas.width;
  const height = canvas.height;

  const projection = geoStereographic()
    .scale(width / 1.3 / 3.14)
    .translate([width / 2, height / 2])
    .clipAngle(90)
    .precision(10);

  const path = geoPath(projection, context);
  context.strokeStyle = "#69b2a2";
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();
}
