function render_edges_from_pt_adj(ctx, mesh) {
  const { adj, points, halfedges, triangles } = mesh;
  let n = adj.length;
  let p = Math.floor(rand(0.1 * n, 0.9 * n));
  let vseen = []
  vseen[p] = 0
  let queue = [p]
  let dist = 0;
  while (queue.length > 0) {

    let p = queue.shift()
    let d = vseen[p]

    ctx.beginPath()
    for (let i = 0, a = adj[p]; a && i < a.length; i++) {
      if (a === undefined) break
      const e = a[i];
      ctx.moveTo(points[p * 2], points[p * 2 + 1])
      ctx.lineTo(points[e * 2], points[e * 2 + 1])
      if (vseen[e] === undefined) {
        vseen[e] = d + 1;
        queue.push(e)
      }
    }
    ctx.stroke()
  }
  return vseen
}

// BFS over halfedges
function render_edges_from_pt(ctx, mesh, draw) {
  const { points, halfedges, triangles } = mesh;
  let n = halfedges.length;
  let p = Math.floor(rand(1, n - 1));
  let seen = new Map([[p, 1]])
  let verts = new Map()
  let queue = [p]
  let dist = 0;
  while (queue.length > 0) {
    dist += 1;
    let e = queue.shift();  // edge
    let d = seen.get(e) // distance
    if (!verts.has(triangles[e])) {
      verts.set(triangles[e], d)
    }

    let en = nextEdge(e)
    let eh = halfedges[e];
    let ep = prevEdge(e)

    if (draw && !seen.has(eh)) {
      ctx.beginPath()
      let color = d3.interpolateRdYlGn(1 - d / (n * 0.1))
      // let color = d3.interpolateRdYlGn(Math.log2(d) / 20)
      ctx.strokeStyle = color
      const t1 = triangles[e] * 2;
      const t2 = triangles[eh] * 2;
      ctx.moveTo(points[t1], points[t1 + 1])
      ctx.lineTo(points[t2], points[t2 + 1])
      ctx.fillText(d, (points[t2] + 2.5 * points[t1]) / 3.5, (points[t2 + 1] + 2.5 * points[t1 + 1]) / 3.5)
      ctx.stroke();
    }
    if (!seen.has(en)) {
      seen.set(en, d + 1)
      queue.push(en);
    }
    if (!seen.has(eh) && eh > -1) {
      seen.set(eh, d + 1)
      queue.push(eh);
    }
    if (!seen.has(ep)) {
      seen.set(ep, d + 1)
      queue.push(ep);
    }
  }
  return verts
}

function render_edges(ctx, vor) {
  console.log(vor);
  const delaunay = vor.delaunay;
  console.log(delaunay);
  const { points, halfedges, triangles } = vor.delaunay;
  console.log(halfedges.length);
  let n = halfedges.length;

  ctx.beginPath();
  let i = 0;
  for (; i < n / 2; i += 1) {

    ctx.strokeStyle = '#006000';
    // let e = halfedges[i]
    let e = i
    if (e < 0) continue
    let f = nextEdge(e)

    const t1 = triangles[e] * 2
    const x1 = points[t1]
    const y1 = points[t1 + 1]

    const t2 = triangles[f] * 2 // points are flat
    const x2 = points[t2]
    const y2 = points[t2 + 1]
    console.log(x1, y1, x2, y2);
    ctx.moveTo(x1, y1)
    ctx.lineTo(x2, y2)

  }
  ctx.stroke();
}

