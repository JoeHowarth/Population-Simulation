import {add, rand, randDir, rnorm} from './map-utils'
import * as d3 from 'd3'
import {mergeSegments} from './render/render-map'


async function genHM(mesh) {
  // let mesh = generateGoodMesh(params.npts, params.extent);
  const [W, H] = mesh.Dkm

  const area = W * H
  const sharpHillDensity = 0.004
  const roundHillDensity = 0.0015
  const mountainDensity = 0.001

  const shNum = Math.ceil(sharpHillDensity * area)
  const rhNum = Math.ceil(roundHillDensity * area)
  const mtNum = Math.ceil(mountainDensity * area)

  console.log('hill nums ', shNum, rhNum, mtNum)


  let h = add(
    slope(mesh, randDir(), 0.5),
    // cone(mesh, rand(-1.0, -0.5) * 0.0005, [rnorm(W/2, W/6), rnorm(H/2, H/6) ]),
    // cone(mesh, 0.01, [rnorm(W/2, W/6), rnorm(H/2, H/6) ]),
    // cone(mesh, 0.010, [rnorm(W/2, W/8), rnorm(H/2, H/8) ]),
    // cone(mesh, 0.005, [rnorm(W/2, W/6), rnorm(H/2, H/6) ]),
    cone(mesh, 0.0015, [rnorm(W / 2, W / 3), rnorm(H / 2, H / 3)]),
    mountains(mesh, shNum, 5, 0.7),
    mountains(mesh, shNum, 5, 0.7),
    mountains(mesh, rhNum, 8, 0.4),
    mountains(mesh, mtNum * 0.5, 30, 0.4),
    mountains(mesh, mtNum * 0.5, 15, 0.8),
    mountains(mesh, mtNum * 0.2, 35, 0.1),
  );
  // console.log("h after mountains", h)
  for (let i = 0; i < 1; i++) {
    h = relax(mesh, h);
  }
  h = peaky(h);
  // h = fillSinks(mesh, h)
  // h = normalize(h);
  // h = doErosion(mesh, h, rand(0.02, 0.1), 2);
  h = doErosion(mesh, h, 0.08, 2);
  // h = setSeaLevel(mesh, h, rand(0.1, 0.2));
  h = setSeaLevel(mesh, h, 0.35);
  h = normalize(h, 0.01)

  h = fillSinks(mesh, h);
  h = cleanCoast(mesh, h, 3);

  mesh.slope = getSlope(mesh, h);
  mesh.flux = getFlux(mesh, h);
  mesh.downhill = downhill(mesh, h);
  mesh.ER = erosionRate(mesh, h);
  quick_stats(mesh.flux, "flux");
  quick_stats(mesh.slope, "slope");

  return h;

}


function relax(mesh, h) {
  let newh = mesh.zero()
  for (let i = 0; i < newh.length; i++) {
    const nbs = mesh.adj[i];
    if (nbs.length < 3 && nbs.length > 0) {
      let di = nbs.slice()
      di.push(i)
      newh[i] = d3.mean(di.map(v => h[v] * 0.99));
      continue;
    } else if (nbs.length < 3) {
      newh[i] = 0;
      continue;
    }
    newh[i] = d3.mean(nbs.map(j => h[j]))
  }
  return newh;
}

function mountains(mesh, n, r, h) {
  r = r || 100;
  h = h || 1;
  let mounts = [];
  const { centroids } = mesh
  const [Wkm, Hkm] = mesh.Dkm
  for (let i = 0; i < n; i++) {
    mounts.push([rand(Wkm * 0.05, Wkm * 0.95), rand(Hkm * 0.05, Hkm * 0.95)]);
  }


  return mesh.zero()
    .map((_, i) => {
      let p = centroids[i]
      let sum = 0
      for (let j = 0; j < n; j++) {
        let m = mounts[j];
        const dist = ((p[0] - m[0]) * (p[0] - m[0]) + (p[1] - m[1]) * (p[1] - m[1]))
        const exp = Math.exp(-dist / (2 * r * r))
        sum += Math.pow(exp * h, 2)
      }
      return sum
    });
}

function slope(mesh, dir, steepness = 1.0) {
  // unit vector
  let len = Math.sqrt(dir[0] * dir[0] + dir[1] * dir[1])
  dir = [dir[0] / len, dir[1] / len]
  let h = mesh.map((pt, i) => {
    // dir is unit vec now
    return pt[0] * dir[0] + pt[1] * dir[1]
  })
  return scale(normalize(h), steepness)

}


function cone(mesh, slope, loc) {
  let [Cx, Cy] = mesh.Dkm.slice()
  Cx /= 2;
  Cy /= 2;
  [Cx, Cy] = loc ? loc : [Cx, Cy]
  return mesh.map(v => {
    const x = (v[0] - Cx)
    const y = (v[1] - Cy)
    const dist = x * x + y * y
    return Math.pow(dist, 0.5) * slope;
  });
}

function normalize(h, lo, hi) {
  if (!lo && lo !== 0) {
    lo = d3.min(h)
  }
  hi = hi || d3.max(h);
  return h.map(x => (x - lo) / (hi - lo))
}

function scale(h, v) {
  return h.map(x => x * v)
}

function peaky(h) {
  return normalize(h)
    .map(Math.sqrt);
}


function downhill(mesh, h) {
  // if (h.downhill) return h.downhill;
  // console.log("h in downHill", h)

  function downFrom(i) {
    if (mesh.isEdge(i)) return -2
    let best = -1
    let besth = h[i]
    let nbs = mesh.adj[i]
    for (let j = 0; j < nbs.length; j++) {
      let n = nbs[j]

      if (h[n] < besth) {
        besth = h[n]
        best = n
      }
    }
    return best
  }

  let downs = h.map((v, i) => downFrom(i))
  h.downhill = downs
  return downs
}

function getFlux(mesh, h) {
  // console.log("h in getflux", h)
  const dh = downhill(mesh, h);
  let idxs = mesh.triIDs.map((v, i) => i)
  const amt = (mesh.Dkm[0] * mesh.Dkm[1]) / h.length * 0.01 * 0.01
  // const amt = 1 / h.length
  let flux = h.map((_, i) => amt)
  idxs.sort((a, b) => h[b] - h[a])

  idxs.forEach((j, i) => {
    if (dh[j] >= 0) {
      flux[dh[j]] += flux[j]
    }
  })
  // quick_stats(flux, "flux")
  console.log("95th", quantile(flux, 0.95))
  return flux
}

// redefining slope to use with tri-vertices not triangles
function getSlope(mesh, h) {
  let dh = downhill(mesh, h);
  const delta = quantile(h, 0.35);
  return mesh.zero()
    .map((v, i) => {
      let s = mesh.trislope(h, i);
      // console.log(s)
      let slope = Math.sqrt(s[0] * s[0] + s[1] * s[1]);
      if (h[i] < delta) {
        slope *= 0.5
      }

      if (slope > 3) {
        return Math.min(Math.max(Math.sqrt(0.7 * slope), 3), 7)
      }
      return slope;
      // if (dh[i] < 0) {
      //   return 0;
      // } else {
      //   // console.log(h[i] - h[dh[i]], i, dh[i], mesh.distance(i, dh[i]))
      //   return (h[i] - h[dh[i]]) / mesh.distance(i, dh[i]);
      // }
    });
}


function erosionRate(mesh, h) {
  let flux = getFlux(mesh, h);
  let slope = getSlope(mesh, h);
  let newh = mesh.zero()
  for (let i = 0; i < newh.length; i++) {
    let river = Math.sqrt(flux[i]) * slope[i];
    let creep = slope[i] * slope[i];
    let total = 1000 * river + creep;
    total = total > 120 ? 120 : total;
    newh[i] = total;
  }
  return newh;
}

function erode(mesh, h, amount) {
  let er = erosionRate(mesh, h);
  let maxr = d3.max(er);

  return mesh.zero()
    .map((v, i) => h[i] - amount * (er[i] / maxr));
}

function doErosion(mesh, h, amount, n) {
  n = n || 1;
  h = fillSinks(mesh, h);
  for (let i = 0; i < n; i++) {
    h = erode(mesh, h, amount);
    h = fillSinks(mesh, h);
  }
  return h;
}

function findSinks(mesh, h) {
  let dh = downhill(h);
  let sinks = mesh.triIDs.map((v, i) => {
    let node = i;
    while (true) {
      if (mesh.isEdge(node)) {
        sinks[i] = -2;
        break;
      }
      if (dh[node] === -1) {
        sinks[i] = node;
        break;
      }
      node = dh[node];
    }
  });
  return sinks
}

function fillSinks(mesh, h, epsilon) {
  epsilon = epsilon || 1e-5;
  let infinity = 999999;
  let newh = mesh.zero()
    .map((v, i) => {
      if (mesh.isNearEdge(i)) {
        return h[i];
      } else {
        return infinity;
      }
    })
  // console.log("fillSinks, init newh", newh)
  // fixed point alg
  const MAX_ITERS = 2000
  for (let iter = 0; iter < MAX_ITERS; iter++) {
    let changed = false;
    for (let i = 0; i < newh.length; i++) {
      if (newh[i] === h[i]) continue;
      let nbs = mesh.adj[i]
      for (let j = 0; j < nbs.length; j++) {
        if (h[i] >= newh[nbs[j]] + epsilon) {
          newh[i] = h[i];
          changed = true;
          break;
        }
        let oh = newh[nbs[j]] + epsilon;
        if ((newh[i] > oh) && (oh > h[i])) {
          newh[i] = oh;
          changed = true;
        }
      }
    }
    if (!changed) return newh;
  }
}

function setSeaLevel(mesh, h, q) {
  let newh = mesh.zero()
  let delta = quantile(h, q);
  console.log('delta', delta)
  console.log('min, max', d3.min(h), d3.max(h))
  for (let i = 0; i < newh.length; i++) {
    newh[i] = h[i] - delta;
  }
  console.log('min, max', d3.min(newh), d3.max(newh))
  return newh;
}

function getRivers(mesh, h, limit) {
  let dh = downhill(mesh, h);
  let flux = getFlux(mesh, h);
  let links = [];
  let above = 0;
  for (let i = 0; i < h.length; i++) {
    if (h[i] > 0) above++;
  }
  limit *= above / h.length;
  for (let i = 0; i < dh.length; i++) {
    if (mesh.isNearEdge(i)) continue;
    if (flux[i] > limit && h[i] > 0 && dh[i] >= 0) {
      let up = i;
      let down = dh[i];
      if (h[dh[i]] > 0) {
        links.push([up, down]);
      } else {
        links.push([up, down]);
        // links.push([up, [(up[0] + down[0]) / 2, (up[1] + down[1]) / 2]]);
      }
    }
  }
  return mergeSegments(links)
  // .map(relaxPath);
}

function cityScore(mesh, h, cities) {
  let coastal = isCoastal(mesh, h);
  let score = mesh.flux.map(Math.sqrt).map((s,i, flux) => {
    let nbs = mesh.adj[i].map(j => flux[j])
    if (coastal[i]) {
      return  0.7 * d3.max(nbs) + 0.3 * s
    }
    return  0.6 * d3.mean(nbs) + 0.4 * s
  });

  for (let i = 0; i < h.length; i++) {
    if (h[i] <= 0 || mesh.isNearEdge( i, 2.5 / mesh.Dkm[0])) {
      score[i] = -999999;
      continue;
    }
    // don't prefer edges of map
    // score[i] += 0.01 / (1e-9 + Math.abs(mesh.normPts[i][0]) - mesh.extent.width / 2)
    // score[i] += 0.01 / (1e-9 + Math.abs(h.mesh.vxs[i][1]) - h.mesh.extent.height / 2)
    for (let j = 0; j < cities.length; j++) {
      score[i] -= (1.82 - Math.sqrt(cities.length) * 0.3) / (mesh.distance(cities[j], i) + 1e-9);
    }
  }
  return score;
}

function isCoastal(mesh, h) {
  return h.map((v, i) => {
    return v > 0
      && mesh.adj[i].some(i => h[i] < 0);
  })
}

function placeCity(mesh, h, cities) {
  cities = cities || [];
  let score = cityScore(mesh, h, cities);
  let newcity = d3.scan(score, d3.descending);
  // console.log("new city: ", newcity, score[newcity])
  cities.push(newcity);
  return cities
}

function placeCities(mesh, h, numCities) {
  let n = numCities;
  let cities = []
  for (let i = 0; i < n; i++) {
    placeCity(mesh, h, cities);
  }
  return cities
}

function cleanCoast(mesh, h, iters) {
  for (let iter = 0; iter < iters; iter++) {
    let changed = 0;
    let newh = mesh.zero();
    for (let i = 0; i < h.length; i++) {
      newh[i] = h[i];
      let nbs = mesh.adj[i]
      if (h[i] <= 0 || nbs.length !== 3) continue;
      let count = 0;
      let best = -999999;
      for (let j = 0; j < nbs.length; j++) {
        if (h[nbs[j]] > 0) {
          count++;
        } else if (h[nbs[j]] > best) {
          best = h[nbs[j]];
        }
      }
      if (count > 1) continue;
      newh[i] = best / 2;
      changed++;
    }
    h = newh;
    newh = mesh.zero()
    for (let i = 0; i < h.length; i++) {
      newh[i] = h[i];
      let nbs = mesh.adj[i]
      if (h[i] > 0 || nbs.length !== 3) continue;
      let count = 0;
      let best = 999999;
      for (let j = 0; j < nbs.length; j++) {
        if (h[nbs[j]] <= 0) {
          count++;
        } else if (h[nbs[j]] < best) {
          best = h[nbs[j]];
        }
      }
      if (count > 1) continue;
      newh[i] = best / 2;
      changed++;
    }
    h = newh;
  }
  return h;
}

function quantile(h, q) {
  let sortedh = h.slice()
  sortedh.sort(d3.ascending);
  return d3.quantile(sortedh, q);
}

export function quick_stats(h, name) {
  console.log(name, d3.min(h), d3.max(h), d3.mean(h))
  console.log(name + " quantiles", quantile(h, 0.25), d3.median(h), quantile(h, 0.75))
}

export {
  genHM,
  fillSinks,
  doErosion,
  erode,
  erosionRate,
  getFlux,
  normalize,
  peaky,
  getSlope,
  cone,
  downhill,
  mountains,
  quantile,
  setSeaLevel,
  cleanCoast,
  getRivers,
  placeCities,
  cityScore,
}
