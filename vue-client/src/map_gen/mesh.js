import * as d3 from 'd3'
import { drawPoly } from './map-utils'
import { genRenderFns, heightToColor } from './render/render-map'
import { normalize } from './heightmap'

const defaultExtent = [[0, 0], [800, 500]]

const angCutoff = 3.14 * 0.7

/*
 * Delaunator DS
 * edge -> triangleID: edge2tri(e) [ -> invTriIDs[ <> ] ]
 * edge -> tri vertex: triangles[e] -> mesh.point( <> )
 * edge -> other edge: halfedge[e]
 */

async function makeMesh(vor, ctx, [Wkm, Hkm], [Wpx, Hpx]) {

  console.time('makeMesh')

  let mesh = Object.create(vor.__proto__);
  Object.assign(mesh, vor);
  Object.assign(mesh, vor.delaunay)
  const { halfedges, points, triangles } = mesh

  mesh.Dkm = [Wkm, Hkm]
  mesh.Dpx = [Wpx, Hpx]
  mesh.km2px = Wpx / Wkm
  mesh.px2km = Wkm / Wpx
  mesh.extent = [+vor.xmax - +vor.xmin, +vor.ymax - +vor.ymin]
  mesh.ctx = ctx

  mesh.pt_km2px = ([x, y]) => [x * mesh.km2px, y * mesh.km2px]


  // calculate valid triangle ids (ie not slivers)
  let [triIDs, invTriIDs] = calcTriIDs(mesh)
  mesh.triIDs = triIDs
  mesh.invTriIDs = invTriIDs
  console.log(invTriIDs)
  console.table(mesh.invTriIDs)


  // adjacent tris to each tri
  let adj = calcAdj(mesh)
  mesh.adj = adj


  console.time('everything else in makeMesh')
  mesh.zero = () => {
    let arr = new Array(mesh.triIDs.length)
    arr.fill(0.0)
    return arr;
  }

  // get i-th triangle vertex
  // associated with mesh.triangles[] NOT new triIDs
  mesh.point_km = i => {
    return [points[i * 2], points[i * 2 + 1]]
  }

  mesh.points_px = mesh.points.map(x => x * mesh.km2px)
  mesh.point_px = i => {
    return [mesh.points_px[i * 2], mesh.points_px[i * 2 + 1]]
  }

  mesh.triPaths = Array.from(mesh.triIDs)
    .map(v => mesh.delaunay.trianglePolygon(v))
  mesh.hullPoly = mesh.delaunay.hullPolygon()

  let centroids = mesh.triPaths.map(centroid)
  let centroids_px = centroids.map(mesh.pt_km2px)

  mesh.vorCentroids = Array.from(mesh.cellPolygons())
    .map(centroid)

  mesh.centroids = centroids
  mesh.centroids_px = centroids_px
  mesh.normPts = calcNormPts(mesh)


  mesh.map = (f) => {
    let mapped = new Array(mesh.adj.length)
    for (let i = 0; i < mapped.length; i++) {
      mapped[i] = f(centroids[i], i, mesh)
    }
    mapped.mesh = mesh;
    return mapped;
  }

  mesh.isEdge = i => !adj[i] || adj[i].length < 3
  mesh.isNearEdge = (i, thresh = 0.05) => {
    let c = mesh.centroids[i]
    if (!c) return
    let [x,y] = c
    let [w, h] = mesh.Dkm;
    return mesh.isEdge(i) || x < thresh * w || x > (1.0 - thresh) * w || y < thresh * h || y > (1.0 - thresh) * h;
  }


  mesh.distance = (i, j) => {
    const [ix, iy] = mesh.centroids[i]
    const [jx, jy] = mesh.centroids[j]
    return Math.sqrt((ix - jx) * (ix - jx) + (iy - jy) * (iy - jy))
  }

  // TODO use km not norm
  mesh.trislope = triSlope(mesh)


  const triPathsPx = mesh.triPaths.map(mesh.pt_km2px)
  mesh = genRenderFns(mesh, ctx)


  console.timeEnd('everything else in makeMesh')
  console.timeEnd('makeMesh')
  console.log(mesh)
  return mesh;

}


/// re-index based off triIDs instead of original
function calcTriIDs(mesh) {
  console.time('goodTris')
  let goodTris = []
  Array.from(mesh.delaunay.trianglePolygons())
    .forEach((pts, i) => {
      const [a, b, c] = pts;

      let a2 = lengthSquared(b, c)
      let b2 = lengthSquared(a, c)
      let c2 = lengthSquared(a, b)

      let al = Math.sqrt(a2)
      let bl = Math.sqrt(b2)
      let cl = Math.sqrt(c2)

      let alpha = Math.acos((b2 + c2 - a2) / (2 * bl * cl))
      let beta = Math.acos((a2 + c2 - b2) / (2 * al * cl))
      let gamma = Math.acos((a2 + b2 - c2) / (2 * al * bl))

      if (alpha < angCutoff && gamma < angCutoff && beta < angCutoff) {
        goodTris.push(i)
      }
    })

  let triIDs = new Uint32Array(goodTris)
  let invTriIds = new Map()
  for (let i = 0; i < triIDs.length; i++) {
    invTriIds.set(triIDs[i], i)
  }

  console.timeEnd('goodTris')
  return [triIDs, invTriIds]
}

function calcAdj(mesh) {
  const { halfedges, invTriIDs } = mesh
  let adj = []
  console.time('adj')
  for (let i = 0; i < halfedges.length; i++) {
    let e0 = i;
    let t0_ = invTriIDs.get(edge2tri(i))
    if (t0_ === undefined) continue

    let e1 = halfedges[e0]
    if (e1 === -1) continue

    let t1_ = invTriIDs.get(edge2tri(e1))
    if (t1_ === undefined) continue

    adj[t0_] = adj[t0_] || [];
    if (!adj[t0_].includes(t1_)) {
      adj[t0_].push(t1_);
    }
    adj[t1_] = adj[t1_] || [];
    if (!adj[t1_].includes(t0_)) {
      adj[t1_].push(t0_);
    }
  }
  console.timeEnd('adj')
  return adj
}

function triSlope(mesh) {
  return (h, i) => {
    let nbs = mesh.adj[i]
    if (nbs.length !== 3) return [0, 0];
    let p0 = mesh.normPts[nbs[0]];
    let p1 = mesh.normPts[nbs[1]];
    let p2 = mesh.normPts[nbs[2]];

    let x1 = p1[0] - p0[0];
    let x2 = p2[0] - p0[0];
    let y1 = p1[1] - p0[1];
    let y2 = p2[1] - p0[1];

    let det = x1 * y2 - x2 * y1;
    let h1 = h[nbs[1]] - h[nbs[0]];
    let h2 = h[nbs[2]] - h[nbs[0]];

    return [(y2 * h1 - y1 * h2) / det,
      (-x2 * h1 + x1 * h2) / det];
  }
}

function calcNormPts(mesh) {
  let xs = []
  let ys = []

  mesh.centroids.forEach(([x, y]) => {
    xs.push(x);
    ys.push(y);
  })
  xs = xs.slice()
  ys = ys.slice()
  let max = d3.max(xs.slice()
    .concat(ys))
  xs = normalize(xs, 0, max)
  ys = normalize(ys, 0, max)

  let normPts = xs.map((x, i) => [x, ys[i]])
  return normPts.map(v => [v[0] * 2.0 - 1.0, v[1] * 2 - 1])
}

function lengthSquared([ax, ay], [bx, by]) {
  let dx = ax - bx
  let dy = ay - by
  return dx * dx + dy * dy
}

// half edge id to triangle id
function edge2tri(e) {
  return Math.floor(e / 3)
}

// triangle id to array of half edge ids
function tri2edge(t) {
  const e0 = t * 3;
  return [e0, e0 + 1, e0 + 2];
}

function nextEdge(e) {
  return (e % 3 === 2) ? e - 2 : e + 1
}

function prevEdge(e) {
  return (e % 3 === 0) ? e + 2 : e - 1
}

function centroid(pts) {
  let x = 0;
  let y = 0;
  for (let i = 0; i < pts.length; i++) {
    x += pts[i][0];
    y += pts[i][1];
  }
  return [x / pts.length, y / pts.length];
}

export { nextEdge, prevEdge, tri2edge, edge2tri, makeMesh };


