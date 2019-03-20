// @ts-ignore
import { Delaunay } from 'd3-delaunay';
import poissonDiscSampler from './poissonDiscSampler';
import { genHM, peaky } from './heightmap';
// @ts-ignore
import { setup_canvas } from './render/webgl';
import { makeMesh, mesh_from_data } from "./mesh";
import store from '../store/store';
import { pt2triangle, } from "./planar-point-by-vec";
// @ts-ignore
import { cities } from './render/render-map';
export const WEBGL = true;
export const RIVER_THRESH = 0.006; //flux
let x = 0;
var canvas, ctx, sampler, vor, Wpx, Hpx, Wkm, Hkm;
export let mesh, h = [];
export function getHeight() {
    return mesh.h.slice();
}
export function getMesh() {
    return mesh;
}
export function setMesh(_mesh) {
    _mesh.invTriIDs = new Map(_mesh.invTriIDs);
    mesh = mesh_from_data(_mesh);
}
export function getER() {
    return peaky(mesh.ER);
}
/*
 * NOTE: km scale probably wrong given geography, rivers, cities etc.
 *       will double (?)
 */
export default async function () {
    console.log("hi from before setup");
    mesh = await setup(100, 100, 2.7);
    // const {points, triangles, halfedges} = mesh
    h = await genHM(mesh);
    // exportMesh(mesh)
    // setTimeout(() => renderMapGL(mesh, h), 0)
    console.log("num_tris", mesh.triIDs.length);
    return [mesh, h];
}
;
/*
 *  Tile/Tri under mouse
 */
export function tri_under_mouse() {
    console.time("tri_under_mouse");
    console.time("pick");
    const scene = window.scene;
    const { hit, pickedPoint, PickedMesh } = scene.pick(scene.pointerX, scene.pointerY);
    console.timeEnd("pick");
    if (hit) {
        var { x, y } = pickedPoint;
    }
    const cam = scene.activeCamera;
    // console.log(cam)
    // console.log(scene.pointerX, scene.pointerY)
    // console.log(x, y)
    console.time("math");
    const p = screen2world();
    console.timeEnd("math");
    console.log();
    const tri = pt2triangle(mesh, [x, y]);
    console.timeEnd("tri_under_mouse");
    return tri;
}
function screen2world() {
    const { scene } = window;
    const x = scene.pointerX;
    const y = scene.pointerY;
    const cam = scene.activeCamera;
    const w = window.canvas.width;
    const h = window.canvas.height;
    // point fraction
    const frac = { x: x / w, y: y / h };
    console.log(frac);
    // upper left corner (origin)
    const orig = {
        x: cam.position.x + cam.orthoLeft,
        y: cam.position.y + cam.orthoTop
    };
    const point = {
        x: orig.x + cam.orthoRight * frac.x * 2,
        y: orig.y + cam.orthoBottom * frac.y * 2
    };
    return point;
}
/* gets canvas ctx, generates points, sets scale transforms
 * width & height: 100 km
 *
 */
async function setup(Wkm_ = 100, Hkm_ = 100, density = 1) {
    Wkm = Wkm_;
    Hkm = Hkm_;
    setup_canvas(Wkm, Hkm);
    console.log('Wkm, Hkm', Wkm, Hkm);
    sampler = poissonDiscSampler(Wkm * 0.98, Hkm * 0.98, density);
    console.time("sample points");
    let points = [];
    const max_points = 10000000;
    // let pts = await getPoisson(num_per_gen, sampler);
    let i = 0;
    for (let s; i < max_points && (s = sampler()); i++) {
        points.push(s);
    }
    console.timeEnd("sample points");
    console.log('num points: ', i);
    points = points.map(([x, y]) => [x + Wkm * 0.010, y + Hkm * 0.010]);
    const delaunay = Delaunay.from(points);
    vor = delaunay.voronoi([0, 0, Wkm, Hkm]);
    return await makeMesh(vor, ctx, [Wkm, Hkm], [Wpx, Hpx]);
}
export function exportMesh(mesh, toFile) {
    // @ts-ignore
    let ret = {};
    ret.adj = Array.from(mesh.adj);
    ret.halfedges = Array.from(mesh.halfedges);
    ret.points = Array.from(mesh.points);
    ret.invTriIDs = Array.from(mesh.invTriIDs);
    ret.triIDs = Array.from(mesh.triIDs);
    ret.Dkm = mesh.Dkm;
    ret.centroids = mesh.centroids;
    ret.triangles = Array.from(mesh.triangles);
    ret.slope = Array.from(mesh.slope);
    ret.flux = Array.from(mesh.flux);
    ret.downhill = Array.from(mesh.downhill);
    ret.h = Array.from(getHeight());
    ret.area = Array.from(mesh.area);
    ret.triPaths = mesh.triPaths;
    ret.hullPoly = mesh.hullPoly;
    // ret.VorCentroids = mesh.VorCentroids
    let dataStr = JSON.stringify({ Mesh: ret });
    if (toFile) {
        exportToJsonFile(dataStr);
    }
    else {
        store.dispatch('sendMessage', dataStr);
    }
}
export function exportCities(file) {
    let ret = {};
    ret['ids'] = cities;
    let dataStr = JSON.stringify({ Mesh: ret });
    if (file) {
        exportToJsonFile(dataStr);
    }
    else {
        store.dispatch('sendMessage', dataStr);
    }
}
function exportToJsonFile(dataStr) {
    let names = [
        "Joe", "Robert", "Peter", "Louise",
        "Doug", "Maximilian", "Vlad", "Luke",
        "Alex", "Devon", "Tyler", "Sam",
        "PeterSmith", "Smith", "Susan",
    ];
    let name1 = names[Math.floor(Math.random() * (names.length - 1))];
    let name2 = names[Math.floor(Math.random() * (names.length - 1))];
    const filename = "map_" + name1 + "_" + name2 + ".json";
    var url = URL.createObjectURL(new Blob([dataStr], { type: 'text/plain' }));
    let linkElement = document.createElement('a');
    linkElement.setAttribute('href', url);
    linkElement.setAttribute('download', filename);
    linkElement.click();
    URL.revokeObjectURL(url);
}
function getMousePos(canvas, evt) {
    var rect = canvas.getBoundingClientRect(), // abs. size of element
    scaleX = canvas.width / rect.width, // relationship bitmap vs. element for X
    scaleY = canvas.height / rect.height; // relationship bitmap vs. element for Y
    return {
        x: (evt.clientX - rect.left) * scaleX,
        y: (evt.clientY - rect.top) * scaleY // been adjusted to be relative to element
    };
}
//# sourceMappingURL=map_gen.js.map