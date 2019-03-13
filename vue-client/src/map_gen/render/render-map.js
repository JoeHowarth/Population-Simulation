import * as d3 from 'd3'
import {drawPoly} from '../map-utils'
import {edge2tri} from '../mesh'
import {getRivers, placeCities} from '../heightmap'
import * as color from './color'
import {drawText} from './glText'
import line2D from './glLine2D'
import * as BABYLON from 'babylonjs'
import {init_babylon} from "./webgl";
import {RIVER_THRESH} from "../map_gen";

export var showCities
export var cities

export async function renderMapGL(mesh, h) {
  let scene = await init_babylon(mesh, h)

  await renderRiversGL(mesh, h, RIVER_THRESH, scene)
  renderCoastLine(mesh, h, 0, true, BABYLON.Color3.Black())

  showCities = () => {
    cities = placeCities(mesh, h, 20)
    // exportCities()
    renderCitiesGL(mesh, cities, 10)
  }

  // setTimeout(showCities, 1)
  // setTimeout(() => displayIDs(mesh), 0);

  /*
  setTimeout(() => {
    let box = BABYLON.MeshBuilder.CreatePlane("", {width: 0.9, height: 0.9}, window.scene);

    canvas.addEventListener("click", (e) => {
      const scene = window.scene
      const {hit, pickedPoint, pickedMesh} = scene.pick(scene.pointerX, scene.pointerY);
      if (hit) {
        var X = pickedPoint.x
        var Y = pickedPoint.y
      }

      console.log('dist from last Q', dist_from_last_query([X, Y]))


      let t = pt2triangle(mesh, [X, Y], box2)

      let highlight = h.slice()
      highlight[t] = 1.0
      updateColorsFun(highlight)


      const [x, y] = mesh.centroids[t]
      box.position = new BABYLON.Vector3(x, y, -3);
    }, {capture: true})

  }, 2);
  */


  renderCoastLine(mesh, h, 0.20, true)
}

const land = d3.interpolateRgbBasis([
  '#A4D475', //
  '#ACCA89',
  '#FFF1C0',
  '#FCA666',
  '#c54902', // dark red
])
const land2 = d3.interpolateRgbBasis([
  '#94BC71',
  '#A1C37F',
  '#BFCF9D',
  '#E0DFBB',
  // '#EFC297',
  '#DF9666',
])
const land2num = color.rgbBasis([
  '#94BC71',
  '#A1C37F',
  '#BFCF9D',
  '#E0DFBB',
  // '#EFC297',
  '#DF9666',
])
const water = d3.interpolateRgb('#0084b8', '#0003b8')
const water_num = color.rgb('#0084b8', '#0003b8')

export function heightToColorArr(h) {
  if (h >= 0) {
    return land2num(h)
  }
  return water_num(-h * 2)
}

function heightToColor(h) {
  return heightToColorArr((h)) + '';
}


function renderRivers(mesh, h, limit) {
  const ctx = mesh.ctx
  ctx.save();
  ctx.scale(mesh.km2px, mesh.km2px)
  ctx.lineWidth *= mesh.px2km
  let paths = getRivers(mesh, h, limit);
  paths = paths.map(p => p.map(i => mesh.centroids[i]))
  paths = paths.map(relaxPath)
  paths.forEach(path => {
    ctx.beginPath()
    const [x, y] = path[0]
    ctx.moveTo(x, y)

    for (let i = 1; i < path.length; i++) {
      let [x, y] = path[i]
      ctx.lineTo(x, y)
    }
    ctx.stroke()
  })
  ctx.restore()
}

export async function renderRiversGL(mesh, h, limit, scene) {
  let paths = getRivers(mesh, h, limit);

  paths = paths.map(p => relaxPath(p.map(i => mesh.centroids[i])))

  let lines = paths.map(path => {
    return path.map(([x, y]) => new BABYLON.Vector3(x, y, -1))
  })

  let lineSystem = BABYLON.MeshBuilder.CreateLineSystem('rivers', {lines}, scene)

  lineSystem.color = BABYLON.Color3.Blue()


  /*
  let major_lines = major_paths.map(path => {
    return path.map(([x, y]) => new BABYLON.Vector3(x, y, -1))
  })
  // let lineSystemMajor = BABYLON.MeshBuilder.CreateLineSystem('rivers', {lines: major_lines}, scene)
  //
  let color = water_num(0.9)
  color = new BABYLON.Color3(color.r / 255, color.g / 255, color.b / 255,)

  console.log(water(0.3))

  let line2DSystem = []
  for (let i = 0; i < major_paths.length; i++) {
    let line = line2D("", {
      path: major_lines[i],
      color: color,
      width: 0.15
    }, window.scene)

    line2DSystem.push(line)
  }
  */


  return lineSystem

}


export async function renderCitiesGL(mesh, cities, res) {
  let sites = cities.map(i => mesh.centroids[i])


  const dim = {
    width: Math.min(mesh.Dkm[0], 512),
    height: Math.min(mesh.Dkm[1], 512),
  }

  const write = drawText(dim, {color: "white", res: res})


  let num_cities = Math.min(sites.length, 5)
  // console.log('num_cities', num_cities)
  for (let i = 0; i < num_cities; i++) {
    let [x, y] = sites[i]

    write("City", x, y, 200)
    let box = BABYLON.MeshBuilder.CreatePlane("city box", {width: 2, height: 2,}, window.scene);
    box.position = new BABYLON.Vector3(x, y, -3);
  }

  for (let i = num_cities; i < cities.length - num_cities; i++) {
    let [x, y] = sites[i]
    write("Town", x, y, 100)
    let box = BABYLON.MeshBuilder.CreatePlane("town box", {width: 1.3, height: 1.3,}, window.scene);
    box.position = new BABYLON.Vector3(x, y, -3);
  }


}

export function displayIDs(mesh) {
  const dim = {
    width: Math.min(mesh.Dkm[0], 512),
    height: Math.min(mesh.Dkm[1], 512),
  }

  const write = drawText(dim, {color: "red", res: 20});

  for (let i = 0; i < mesh.triIDs.length; i++) {
    write(i, mesh.centroids[i][0] - 0.2, mesh.centroids[i][1] - 0.2, 20)
  }
}

function _renderCitiesGL(mesh, cities) {
  let sites = cities.map(i => mesh.centroids[i])


  const dim = {
    width: Math.min(mesh.Dkm[0], 512),
    height: Math.min(mesh.Dkm[1], 512),
  }

  let textPlane = BABYLON.MeshBuilder.CreatePlane('textPlane', dim, window.scene)

  textPlane.material = new BABYLON.StandardMaterial('textPlane', window.scene)
  textPlane.position = new BABYLON.Vector3(0, 0, -3);

  let textText = new BABYLON.DynamicTexture('dyn Text', {
    width: dim.width * 20,
    height: dim.height * 20
  }, window.scene, true)
  let textTextColor = new BABYLON.DynamicTexture('dyn Text', {
    width: dim.width,
    height: dim.height
  }, window.scene, true)
  const ctx = textTextColor.getContext()
  ctx.fillStyle = 'white'
  ctx.fillRect(0, 0, dim.width, dim.height)
  textTextColor.update()

  textPlane.material.opacityTexture = textText
  textPlane.material.diffuseTexture = textTextColor;
  textPlane.material.specularColor = new BABYLON.Color3(0, 0, 0);
  textPlane.material.emissiveColor = new BABYLON.Color3(1, 1, 1);
  textPlane.material.backFaceCulling = false;
  textPlane.position.x = dim.width / 2
  textPlane.position.y = dim.height / 2
  textText.hasAlpha = true


  let num_cities = Math.min(sites.length, 5)
  // console.log('num_cities', num_cities)
  for (let i = 0; i < num_cities; i++) {
    let [x, y] = sites[i]

    // console.log(y * 20)
    // console.log(dim)
    textText.drawText("city", x * 20, (dim.height - y) * 20, "100px sans-serif", "white", null, true)
    // ctx.font = ctx.font.replace(/\d+px/, (parseInt(ctx.font.match(/\d+px/)) - 1) + "px");
    // ctx.fillText(i + 1, x - 8, y - 15)
    let box = BABYLON.MeshBuilder.CreatePlane("city box", {width: 2, height: 2,}, window.scene);
    box.position.x = x
    box.position.y = y
    box.position.z = -3
    // console.log(box)
    // ctx.fillRect(x - 5, y - 5, 10, 10)
  }

  for (let i = num_cities; i < cities.length - num_cities; i++) {
    let [x, y] = sites[i]
    // ctx.font = ctx.font.replace(/\d+px/, (parseInt(ctx.font.match(/\d+px/)) - 1) + "px");
    // ctx.fillText(i + 1, x - 10, y - 10)
    // ctx.fillRect(x - 3, y - 3, 6, 6)
    let box = BABYLON.MeshBuilder.CreatePlane("city box", {width: 1.3, height: 1.3,}, window.scene);
    box.position.x = x
    box.position.y = y
    box.position.z = -3
  }
}

function renderCities(mesh, cities) {
  const ctx = mesh.ctx
  ctx.save();

  // ctx.scale(mesh.km2px, mesh.km2px)
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  // ctx.lineWidth *= mesh.px2km
  let sites = cities.map(i => mesh.centroids_px[i])

  ctx.beginPath()
  ctx.fillStyle = 'white'

  ctx.font = ctx.font.replace(/\d+px/, parseInt(30) + 'px');

  let num_cities = Math.min(sites.length, 5)
  // console.log('num_cities', num_cities)
  for (let i = 0; i < num_cities; i++) {
    let [x, y] = sites[i]

    // ctx.font = ctx.font.replace(/\d+px/, (parseInt(ctx.font.match(/\d+px/)) - 1) + "px");
    ctx.fillText(i + 1, x - 8, y - 15)
    ctx.fillRect(x - 5, y - 5, 10, 10)
  }

  ctx.font = ctx.font.replace(/\d+px/, parseInt(18) + 'px');
  for (let i = num_cities; i < cities.length - num_cities; i++) {
    let [x, y] = sites[i]
    // ctx.font = ctx.font.replace(/\d+px/, (parseInt(ctx.font.match(/\d+px/)) - 1) + "px");
    ctx.fillText(i + 1, x - 10, y - 10)
    ctx.fillRect(x - 3, y - 3, 6, 6)

  }

  ctx.stroke()
  ctx.restore()
}

function relaxPathAmt(path, amt) {
  let newpath = [path[0]];
  let nb = 0.25 * amt
  let cent = 1.0 - 2 * nb
  for (let i = 1; i < path.length - 1; i++) {
    let newpt = [nb * path[i - 1][0] + cent * path[i][0] + nb * path[i + 1][0],
      nb * path[i - 1][1] + cent * path[i][1] + nb * path[i + 1][1]];
    newpath.push(newpt);
  }
  newpath.push(path[path.length - 1]);
  return newpath;
}

function relaxPath(path) {
  let newpath = [path[0]];
  for (let i = 1; i < path.length - 1; i++) {
    let newpt = [0.25 * path[i - 1][0] + 0.5 * path[i][0] + 0.25 * path[i + 1][0],
      0.25 * path[i - 1][1] + 0.5 * path[i][1] + 0.25 * path[i + 1][1]];
    newpath.push(newpt);
  }
  newpath.push(path[path.length - 1]);
  return newpath;
}

function contour(mesh, h, level) {
  level = level || 0;
  let edges = [];
  const {halfedges, invTriIDs, triangles, points, triIDs} = mesh
  let done = [];
  for (let e1 = 0; e1 < halfedges.length; e1++) {

    let e2 = halfedges[e1]
    if (done[e1]) continue
    let t1 = triangles[e1]
    let id1 = invTriIDs.get(edge2tri(e1))

    if (id1 === undefined) continue

    let t2 = triangles[e2]
    let id2 = invTriIDs.get(edge2tri(e2))

    if (e2 !== -1 && id2 === undefined) continue

    if ((h[id1] >= level && h[id2] < level)
      || (h[id2] > level && h[id1] < level)) {
      let p = [t1, t2]
      edges.push(p) // id based, lookup with mesh.point_km
      done[e2] = true
    }

  }

  // console.log('edges', edges)
  // console.time('mergesegments Time')
  let e = mergeSegments(edges);
  // console.timeEnd('mergesegments Time')
  // console.log('after merge', e)
  return e
}

export function renderCoastLine(mesh, h, level = 0, thin, color, smooth) {
  let paths = contour(mesh, h, level)
  paths = smooth ?
    paths.map(p => relaxPathAmt(p.map(i => mesh.point_km(i)), 0.1))
    : paths.map(p => (p.map(i => mesh.point_km(i))))


  let lines = paths.map(path => {
    return path.map(([x, y]) => new BABYLON.Vector3(x, y, -3))
  });

  let lineSystems = []
  if (thin) {
    lineSystems = BABYLON.MeshBuilder.CreateLineSystem('contour', {lines}, scene)
    lineSystems.color = color ? color : new BABYLON.Color3(0.67, 0.67, 0.67)
    // console.log("thin, ", lineSystems)
  }

  else {
    color = color ? color : BABYLON.Color3.Black()
    for (let i = 0; i < lines.length; i++) {
      let lineSystem = line2D("", {
        path: lines[i],
        color,
        width: 0.15
      }, window.scene)
      lineSystems.push(lineSystem)
    }
  }

  return lineSystems
}

function renderCoastLine2d(mesh, h, level = 0) {
  const ctx = mesh.ctx
  ctx.save();

  ctx.scale(mesh.km2px, mesh.km2px)
  ctx.lineWidth *= mesh.px2km
  if (level === 0) {
    ctx.lineWidth *= Math.sqrt(mesh.km2px)
  }

  // console.time('contour')
  let paths = contour(mesh, h, level)
  // console.timeEnd('contour')
  // console.log(paths)

  // console.time('render coastline')
  paths.forEach(path => {

    ctx.beginPath()

    const [x, y] = mesh.point_km(path[0])
    ctx.moveTo(x, y)

    for (let i = 0; i < path.length; i++) {
      let [x, y] = mesh.point_km(path[i])
      ctx.lineTo(x, y)
    }

    ctx.stroke()

  })
  // console.timeEnd('render coastline')

  ctx.restore();
}

function showNeighbors(mesh, i) {
  const ctx = mesh.ctx
  const nbs = mesh.adj[i]
  const [x, y] = mesh.centroids[i]
  nbs.forEach(j => {
    ctx.beginPath()
    ctx.moveTo(x, y)
    const [x1, y1] = mesh.centroids[j]
    ctx.lineTo(x1, y1)
    ctx.stroke()
  })
}

function renderCentroid(mesh) {
  const ctx = mesh.ctx
  ctx.fillStyle = '#880000'
  mesh.triIDs.forEach((_, v) => {
    const [x, y] = mesh.centroids[v]
    ctx.beginPath()

    ctx.fillRect(x - 7, y - 7, 14, 14)
    ctx.fill()

  })
  ctx.fillStyle = '#000000'
}

function genRenderFns(mesh) {
  const {ctx} = mesh
  mesh.renderMesh = (arr, {a, cfn} = {cfn: heightToColor}) => {
    if (arr) {
      ctx.save()
      cfn = cfn || heightToColor
      ctx.globalAlpha = a ? a : 1
      ctx.scale(mesh.km2px, mesh.km2px)
      // draw array
      for (let i = 0; i < arr.length; i++) {
        drawPoly(mesh.ctx, mesh.triPaths[i], cfn(arr[i]))
      }


      ctx.restore()
    }
  }

  mesh.renderDel = ({color, alpha} = {}) => {
    ctx.stroke()
    ctx.strokeStyle = color ? color : 'rgb(0,0,0)'
    ctx.globalAlpha = alpha ? alpha : 1
    const {points_px, halfedges, triangles} = mesh;
    halfedges.forEach((j, i) => {
      if (j < i) return;
      const ti = triangles[i] * 2;
      const tj = triangles[j] * 2;
      ctx.moveTo(points_px[ti], points_px[ti + 1]);
      ctx.lineTo(points_px[tj], points_px[tj + 1]);
    })
    ctx.stroke()
    ctx.globalAlpha = 1
  }

  mesh.renderVor = ({color, alpha, del} = {}) => {
    ctx.save()
    ctx.stroke()
    ctx.strokeStyle = color ? color : 'rgb(0,0,0)'
    ctx.globalAlpha = alpha ? alpha : 1
    ctx.scale(mesh.km2px, mesh.km2px)
    del ? mesh.delaunay.render(ctx) : mesh.render(ctx)
    ctx.stroke()
    ctx.globalAlpha = 1
    ctx.restore()
  }
  return mesh
}

function mergeSegments(segs) {
  // console.log("segs: ",segs)
  let adj = []
  for (let i = 0; i < segs.length; i++) {
    let [id0, id1] = segs[i];

    let a0 = adj[id0] || []
    a0.push(id1)

    let a1 = adj[id1] || []
    a1.push(id0)

    adj[id0] = a0;
    adj[id1] = a1;
  }
  // console.log("adj: ", adj)

  let paths = []
  let done = []
  let path = null
  // build paths
  for (let iter = 0; iter < 20000; iter++) {
    if (path === null) {
      // start new path
      for (let i = 0; i < segs.length; i++) {
        if (done[i]) continue; // skip until find not added segment

        done[i] = true
        path = segs[i].slice()
        // console.log("new path", path)
        break;
      }
      if (path === null) {
        // console.log("done",done)
        return paths; // all done, so return
      }
    }
    // have path, extend it
    let changed = false;
    for (let i = 0; i < segs.length; i++) {
      if (done[i]) continue;
      // console.log(i, segs[i])

      let len = path.length
      // if last element in path has 2 adjacent ids
      if (adj[path[path.length - 1]].length === 2) {
        // if first id in curr segment equal to last in path
        if (segs[i][0] === path[path.length - 1]) {
          // add it
          path.push(segs[i][1])
          // console.log("push", segs[i][1], path)
        } else if (segs[i][1] === path[path.length - 1]) {
          path.push(segs[i][0])
          // console.log("push", segs[i][0], path)
        }
      } else if (adj[path[0]].length === 2) {
        // same for front
        if (segs[i][0] === path[0]) {
          path.unshift(segs[i][1])
          // console.log("unshift", segs[i][1], path)
        } else if (segs[i][1] === path[0]) {
          path.unshift(segs[i][0])
          // console.log("unshift", segs[i][0], path)
        }
      }

      if (len != path.length) {
        changed = true;
        done[i] = true;
        break;
      }
      // none added
      // console.log("continue")

    }
    if (!changed) {
      paths.push(path)
      path = null
    }

  }


}

export {
  showNeighbors,
  renderCentroid,
  heightToColor,
  genRenderFns,
  contour,
  mergeSegments,
  renderCoastLine2d,
  renderRivers,
  renderCities,
}
