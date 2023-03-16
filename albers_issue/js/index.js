
import { geoPath, geoAlbers, geoGraticule, } from "d3-geo";

let grid = document.getElementById("grid");
let width = 1200;
let height = 1200;
const projection = geoAlbers()
  .scale(width / 1.5 / 3.14)
  .translate([width / 2, height / 2]);


const path = geoPath(projection);
const g = geoGraticule().extentMinor([
  [-180, 50 - 1e-6], [180, 60 + 1e-6]
]);

const line_array = g.lines();

// variable for the namespace
const svgns = "http://www.w3.org/2000/svg";
// make a simple rectangle
let path1 = document.createElementNS(svgns, "path");
path1.setAttribute('d', path(line_array[37]));
grid.appendChild(path1);

let path2 = document.createElementNS(svgns, "path");
path2.setAttribute('d', path(line_array[38]));
grid.appendChild(path2);