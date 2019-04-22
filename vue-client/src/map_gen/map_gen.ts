// @ts-ignore
import {Delaunay} from 'd3-delaunay'
import poissonDiscSampler from './poissonDiscSampler'
import {genHM, peaky} from './heightmap'
// @ts-ignore
import {setup_canvas} from './render/webgl'
import {makeMesh, mesh_from_data} from "./mesh"
import store from '../store/store'
import {pt2triangle,} from "./planar-point-by-vec"
// @ts-ignore
import {cities} from './render/render-map'

export const WEBGL = true;
export const RIVER_THRESH = 0.006; //flux


declare global {
  interface Window {
    scene: any,
    engine: any,
    canvas: HTMLCanvasElement,
  }
  
  type CTX = CanvasRenderingContext2D;
  type MapData = number[]
  
  interface Mesh {
    halfedges: number[],
    triangles: number[],
    adj: Array<Array<number>>,
    points: number[],
    Dkm: [number, number],
    triIDs: number[],
    invTriIDs: Map<number, number> | Array<[number, number]>,
    downhill: number[],
    slope: number[],
    flux: number[],
    centroids: number[][],
    h: number[],
    area: number[],
    triPaths: Array<Array<Array<number>>>
    hullPoly: number[][]
    [prop: string]: any,
  }
  
  type Point = [number, number] | number[]
}



let x = 0;

var canvas, ctx: CTX,
  sampler,
  vor,
  Wpx: number, Hpx: number,
  Wkm: number, Hkm: number;

export let mesh: Mesh, h: MapData = [];

export function getHeight(): MapData {
  return mesh.h.slice()
}

export function getMesh(): Mesh {
  return mesh
}

export function setMesh(_mesh: Mesh) {
  _mesh.invTriIDs = new Map(_mesh.invTriIDs)
  mesh = mesh_from_data(_mesh)
}

export function getER(): MapData {
  return peaky(mesh.ER)
}


/*
 * NOTE: km scale probably wrong given geography, rivers, cities etc.
 *       will double (?)
 */
export default async function () {
  
  console.log("hi from before setup")
  mesh = await setup(100, 100, 1.7)
  // const {points, triangles, halfedges} = mesh

  h = await genHM(mesh)
  
  // exportMesh(mesh)

  // setTimeout(() => renderMapGL(mesh, h), 0)
  console.log("num_tris", mesh.triIDs.length)

  return [mesh, h]
};

/*
 *  Tile/Tri under mouse
 */
export function tri_under_mouse() {
  console.time("tri_under_mouse")
  console.time("pick")
  const scene = window.scene
  const {hit, pickedPoint, PickedMesh} = scene.pick(scene.pointerX, scene.pointerY)
  console.timeEnd("pick")
  if (hit) {
    var {x, y} = pickedPoint
  }
  const cam = scene.activeCamera


  // console.log(cam)
  // console.log(scene.pointerX, scene.pointerY)
  // console.log(x, y)
  console.time("math")
  const p = screen2world()
  console.timeEnd("math")

  console.log()

  const tri = pt2triangle(mesh, [x, y])
  console.timeEnd("tri_under_mouse")
  return tri
}

export function screen2world() {
  const {scene} = window
  const x = scene.pointerX
  const y = scene.pointerY
  const cam = scene.activeCamera
  const w = window.canvas.width
  const h = window.canvas.height

  // point fraction
  const frac = {x: x / w, y: y / h}
  // console.log(frac)
  // upper left corner (origin)
  const orig = {
    x: cam.position.x + cam.orthoLeft,
    y: cam.position.y + cam.orthoTop
  }

  return {
    x: orig.x + cam.orthoRight * frac.x * 2,
    y: orig.y + cam.orthoBottom * frac.y * 2
  }
}

/* gets canvas ctx, generates points, sets scale transforms
 * width & height: 100 km
 *
 */
async function setup(Wkm_ = 100, Hkm_ = 100, density = 1) {
  Wkm = Wkm_
  Hkm = Hkm_

  setup_canvas(Wkm, Hkm)

  console.log('Wkm, Hkm', Wkm, Hkm)

  sampler = poissonDiscSampler(Wkm * 0.98, Hkm * 0.98, density);

  console.time("sample points")

  let points = [];
  const max_points = 10000000;
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

  return await makeMesh(vor, ctx, [Wkm, Hkm], [Wpx, Hpx])
}


export function exportMesh(mesh: Mesh, toFile?: boolean) {
  // @ts-ignore
  let ret: Mesh = {}
  ret.adj = Array.from(mesh.adj)
  ret.halfedges = Array.from(mesh.halfedges)
  ret.points = Array.from(mesh.points)
  ret.invTriIDs = Array.from(mesh.invTriIDs)
  ret.triIDs = Array.from(mesh.triIDs)
  ret.Dkm = mesh.Dkm
  ret.centroids = mesh.centroids
  ret.triangles = Array.from(mesh.triangles)
  ret.slope = Array.from(mesh.slope)
  ret.flux = Array.from(mesh.flux)
  ret.downhill = Array.from(mesh.downhill)
  ret.h = Array.from(getHeight())
  ret.area = Array.from(mesh.area)
  ret.triPaths = mesh.triPaths
  ret.hullPoly = mesh.hullPoly
  // ret.VorCentroids = mesh.VorCentroids

  let dataStr = JSON.stringify({Mesh: ret});
  if (toFile) {
    exportToJsonFile(dataStr)
  }
  else {
    store.dispatch('sendMessage', dataStr)
  }
}


export function exportCities(file: string) {
  let ret: any = {}
  ret['ids'] = cities
  let dataStr = JSON.stringify({Mesh: ret});
  if (file) {
    exportToJsonFile(dataStr)
  }
  else {
    store.dispatch('sendMessage', dataStr)
  }

}

function exportToJsonFile(dataStr: string) {
  let names = [
    "Joe", "Robert", "Peter", "Louise",
    "Doug", "Maximilian", "Vlad", "Luke",
    "Alex", "Devon", "Tyler", "Sam",
    "PeterSmith", "Smith", "Susan",
  ]

  let name1 = names[Math.floor(Math.random() * (names.length - 1))];
  let name2 = names[Math.floor(Math.random() * (names.length - 1))];
  const filename = "map_" + name1 + "_" + name2 + ".json"

  var url = URL.createObjectURL(new Blob([dataStr], {type: 'text/plain'}));

  let linkElement = document.createElement('a');
  linkElement.setAttribute('href', url);
  linkElement.setAttribute('download', filename);
  linkElement.click();

  URL.revokeObjectURL(url)
}

function getMousePos(canvas: HTMLCanvasElement, evt: MouseEvent) {
  var rect = canvas.getBoundingClientRect(), // abs. size of element
    scaleX = canvas.width / rect.width,    // relationship bitmap vs. element for X
    scaleY = canvas.height / rect.height;  // relationship bitmap vs. element for Y

  return {
    x: (evt.clientX - rect.left) * scaleX,   // scale mouse coordinates after they have
    y: (evt.clientY - rect.top) * scaleY     // been adjusted to be relative to element
  }
}
