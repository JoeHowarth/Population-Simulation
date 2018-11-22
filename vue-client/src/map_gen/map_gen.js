import {rand, randi, add, drawPoly, alternate} from './map-utils'
import {Delaunay} from 'd3-delaunay';
import * as d3 from 'd3';
import poissonDiscSampler from './poissonDiscSampler'
import * as HM from './heightmap'
import {
  heightToColor,
  renderCities, renderCitiesGL, renderCoastLine,
  renderCoastLine2d,
  renderRivers,
  renderRiversGL
} from './render/render-map'
import {} from './heightmap'
import {
  genHM,
  normalize,
  cityScore,
  placeCities
} from './heightmap'
import {} from './heightmap'
import {init_babylon} from './render/webgl'
import {makeMesh} from "./mesh";



const WEBGL = true;

var canvas,
  sampler,
  ctx,
  vor,
  mesh,
  Wpx,
  Hpx,
  Wkm,
  Hkm;


export default async function (event) {

  mesh = await setup(100, 100, 0.6)
  // const {points, triangles, halfedges} = mesh

  console.log(mesh)
  let m = await genHM(mesh)
  // exportMesh(mesh)

  let scene = await init_babylon(mesh, m)

  await renderRiversGL(mesh, m, 0.01, scene)
  renderCoastLine(mesh, m)
  setTimeout(async () => {
    const cities = placeCities(mesh, m, 20)
    renderCitiesGL(mesh, cities)
  }, 10)


  renderCoastLine(mesh, m, 0.3, true)

  return

  console.log(mesh)

  /*
  let slope = HM.getSlope(mesh, m)
  let slope_vis = normalize(slope)
  console.log("slope info", d3.min(slope), d3.max(slope), d3.median(slope))
  */

  console.log("height info", d3.min(m), d3.max(m), d3.median(m))
  let flux = HM.getFlux(mesh, m)
  console.log("flux info", d3.min(flux), d3.max(flux), d3.median(flux), d3.mean(flux))

  let score = cityScore(mesh, m, []);
  let norm_score = normalize(score, 0.01).map(v => {
    if (v < 0.0) return -0.9;
    if (v < 0.3) return -0.1;

    return Math.sqrt(v);
  })

  if (true) {
    mesh.renderMesh(m)
    // mesh.renderMesh(peaky(flux).map(v => v * 1))
    renderCoastLine2d(mesh, m)
    // renderCoastLine(mesh, m, 0.3)
    // renderCoastLine(mesh, m, 0.6)
    // mesh.renderMesh(norm_score)
    renderRivers(mesh, m, 0.01);
    let cities = placeCities(mesh, m, 20);
    renderCities(mesh, cities)
  } else {
    alternate([
        () => mesh.renderMesh(m),
        // () => mesh.renderMesh(slope_vis),
        // () => mesh.renderMesh(normalize(er)),
      ],
      2000)
  }
};


/* gets canvas ctx, generates points, sets scale transforms
 * width & height: 100 km
 *
 */
async function setup(Wkm_ = 100, Hkm_ = 100, density = 1) {
  Wkm = Wkm_
  Hkm = Hkm_
  const ratio = Wkm / Hkm;
  Hpx = window.outerHeight * 0.85;
  Wpx = Hpx * ratio
  if (Wpx > window.outerWidth * 0.7) {
    Wpx = window.outerWidth * 0.7
    Hpx = Wpx / ratio
  }

  const km2px = Wpx / Wkm
  const px2km = Wkm / Wpx

  canvas = document.getElementById('map_canvas');
  canvas.width = Wpx;
  canvas.height = Hpx;
  canvas.margin = '5px'
  if (!WEBGL) {
    ctx = canvas.getContext('2d');
    ctx.font = '18px serif';
    ctx.strokeRect(0, 0, Wpx, Hpx)
  }

  console.log('Wpx, Hpx', Wpx, Hpx)
  console.log('Wkm, Hkm', Wkm, Hkm)


  sampler = poissonDiscSampler(Wkm * 0.98, Hkm * 0.98, density);

  console.time("sample points")

  let points = [];
  const max_points = 1000000;
  // let pts = await getPoisson(num_per_gen, sampler);
  let i = 0
  for (let s; i < max_points && (s = sampler()); i++) {
    points.push(s)
  }
  console.timeEnd("sample points")
  console.log('num points: ', i)

  points = points.map(([x, y]) => [x + Wkm * 0.010, y + Hkm * 0.010])

  const delaunay = Delaunay.from(points);
  vor = delaunay.voronoi([0, 0, Wkm, Hkm]);

  mesh = await makeMesh(vor, ctx, [Wkm, Hkm], [Wpx, Hpx])

  return mesh

}


function exportMesh(mesh) {
  let ret = {}
  ret.adj = mesh.adj
  ret.points = mesh.points
  ret.invTriIDs = mesh.invTriIDs
  ret.triIDs = mesh.triIDs
  ret.Dkm = mesh.Dkm
  ret.centroids = mesh.centroids
  ret.VorCentroids = mesh.VorCentroids

  // exportToJsonFile(ret)
}

function exportToJsonFile(jsonData) {
  let dataStr = JSON.stringify(jsonData);
  let dataUri = 'data:application/json;charset=utf-8,' + encodeURIComponent(dataStr);

  let exportFileDefaultName = 'data.json';

  let linkElement = document.createElement('a');
  linkElement.setAttribute('href', dataUri);
  linkElement.setAttribute('download', exportFileDefaultName);
  linkElement.click();
}
