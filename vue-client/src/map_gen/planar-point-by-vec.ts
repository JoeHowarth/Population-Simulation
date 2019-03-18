import {lengthSquared} from "./mesh";
import * as d3 from "d3";


let last_tri = 0
// returns index of triangle containing pt
// uses grid to speed up search
export function pt2triangle(mesh, pt) {

  // console.log(distanceSquared(mesh, pt, last_tri), grid_dist)
  if (distanceSquared(mesh, pt, last_tri) > grid_dist) {
    const idx = Math.floor(pt[0] / grid_dist)
    const idy = Math.floor(pt[1] / grid_dist)
    last_tri = grid[idx][idy] === -1 ? last_tri : grid[idx][idy]

  }

  let t = last_tri
  let count = 0
  let {pointInTriangleW, memoVisited} = pointInTriangleMemo(mesh, pt)
  while (!pointInTriangleW(t) && count < 20000) {
    let nbs = mesh.adj[t]
    let min = 888888
    for (let i = 0; i < nbs.length; i++) {
      if (memoVisited[nbs[i]]) continue;

      let d = distanceSquared(mesh, pt, nbs[i])
      if (d < min) {
        min = d
        t = nbs[i]
      }
    }
    count++
  }

  last_tri = t
  return t
}


let grid // hash location to triangle
let grid_dist // maximum distance any point can be from a point in grid

export function init_grid(mesh) {
  grid = [] // stores triangle id
  // const len = mesh.triIDs.length / 50
  const len = 20
  grid_dist = d3.max(mesh.Dkm) / len

  for (let i = 0; i < len; i++) {
    grid.push(new Int32Array(len))
  }
  for (let i = 0; i < mesh.centroids.length; i++) {
    const pt = mesh.centroids[i]
    const idx = Math.floor(pt[0] / grid_dist)
    const idy = Math.floor(pt[1] / grid_dist)
    if (grid[idx][idy] !== -1) {
      grid[idx][idy] = i
    }
  }
}

function pointInTriangleMemo(mesh, pt) {
  let memoVisited = new Uint8Array(mesh.triIDs.length)
  memoVisited.fill(0)


  const pointInTriangleW = t => {
    if (memoVisited[t]) {
      return false
    }

    let [v1, v2, v3] = mesh.triPaths[t]
    // console.log(pt, t, v1, v2, v3)
    let res = pointInTriangle(pt, v1, v2, v3)
    if (!res) {
      memoVisited[t] = 1
    }
    return res
  }

  return {memoVisited, pointInTriangleW}
}

function sign(p1, p2, p3) {
  return (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
}

function pointInTriangle(pt, v1, v2, v3) {

  let d1 = sign(pt, v1, v2);
  let d2 = sign(pt, v2, v3);
  let d3 = sign(pt, v3, v1);

  let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
  let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

  return !(has_neg && has_pos);
}



/*
let last_tri_no_grid = 0
let last_tri_no_grid_anim = 0
let last_tri_anim = 0
*/


function distanceSquared(mesh, [x, y], t) {
  let [x2, y2] = mesh.centroids[t]
  return lengthSquared([x, y], [x2, y2])
}

let last_query = [0,0]
export function dist_from_last_query(pt): number {
  const ret = Math.sqrt(lengthSquared(last_query, pt))
  last_query = pt
  return ret
}

/*
export function pt2triangle_no_grid(mesh, pt) {
  if (!last_tri_no_grid) {
    last_tri_no_grid = 0
  }


  let t = last_tri_no_grid
  let count = 0
  let {pointInTriangleW, memoVisited} = pointInTriangleMemo(mesh, pt)
  while (!pointInTriangleW(t) && count < 20000) {
    let nbs = mesh.adj[t]
    let min = 888888
    for (let i = 0; i < nbs.length; i++) {
      if (memoVisited[nbs[i]]) continue;

      let d = distanceSquared(mesh, pt, nbs[i])
      if (d < min) {
        min = d
        t = nbs[i]
      }
    }
    count++
  }

  last_tri_no_grid = t
  return t
}

export function pt2triangle_grid_animated(mesh, pt, box, speed = 20) {
  // if (!last_tri_no_grid) {
  //   last_tri_no_grid = 0
  // }

  if (distanceSquared(mesh, pt, last_tri_anim) > grid_dist) {
    const idx = Math.floor(pt[0] / grid_dist)
    const idy = Math.floor(pt[1] / grid_dist)
    last_tri_anim = grid[idx][idy] === -1 ? last_tri_anim : grid[idx][idy]

  }

  let t = last_tri_anim
  let count = 0
  let {pointInTriangleW, memoVisited} = pointInTriangleMemo(mesh, pt)
  // while (!pointInTriangleW(t) && count < 20000) {
  const loop = (t) => {

    let nbs = mesh.adj[t]
    let min = 888888
    for (let i = 0; i < nbs.length; i++) {
      if (memoVisited[nbs[i]]) continue;

      let d = distanceSquared(mesh, pt, nbs[i])
      if (d < min) {
        min = d
        t = nbs[i]
      }
    }
    box.position.x = mesh.centroids[t][0]
    box.position.y = mesh.centroids[t][1]

    count++
    if (!pointInTriangleW(t) && count < 20000) {
      setTimeout(() => loop(t),100)

    } else{
      last_tri_anim = t
      return t
    }
  }

  t = loop(t)


  return t

}

export function pt2triangle_animated(mesh, pt, box, speed = 20) {
  // if (!last_tri_no_grid) {
  //   last_tri_no_grid = 0
  // }

  let t = last_tri_no_grid_anim
  let count = 0
  let {pointInTriangleW, memoVisited} = pointInTriangleMemo(mesh, pt)
  // while (!pointInTriangleW(t) && count < 20000) {
  const loop = (t) => {

    let nbs = mesh.adj[t]
    let min = 888888
    for (let i = 0; i < nbs.length; i++) {
      if (memoVisited[nbs[i]]) continue;

      let d = distanceSquared(mesh, pt, nbs[i])
      if (d < min) {
        min = d
        t = nbs[i]
      }
    }
    box.position.x = mesh.centroids[t][0]
    box.position.y = mesh.centroids[t][1]

    count++
    if (!pointInTriangleW(t) && count < 20000) {
      setTimeout(() => loop(t),100)

    } else{
      last_tri_no_grid_anim = t
      return t
    }
  }

  t = loop(t)


  return t

}

export function pt2triangle_naive(mesh, pt) {
  let res = mesh.triPaths.map((path, i) => pointInTriangle(pt, path[0], path[1], path[2]))
  let tri
  for (let i = 0; i < mesh.triPaths.length; i++) {
    let [v1, v2, v3] = mesh.triPaths[i]
    if (pointInTriangle(pt, v1, v2, v3)) {
      tri = i
      break
    }
  }

  return tri
}



*/
