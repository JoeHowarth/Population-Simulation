


pub struct Mesh {
    pub halfedges: Vec<usize>,
    pub triangles: Vec<usize>,
    hull: Vec<usize>,
    pub points: Vec<Vec2>,
    pub dim: Vec2,
    pub adj: Vec<Vec<usize>>,
    pub ids: Vec<usize>,
    pub inv_ids: VecMap<usize>,
//    tri_paths: Vec<[Vec2; 3]>,
}


impl Mesh {
    /// Creates mesh by calcuating poisson points and delaunay triangulation
    pub fn create(w: f32, h: f32, radius: f32) -> Mesh {
        let mut tri = None;
        let mut points = vec![];
        while tri.is_none() {
//            points = get_poisson_points(w, h, radius);
            points = get_poisson_points_js(w, h, radius);
            tri = get_delaunay(&points);
        }

        console::time_with_label("mesh new");
        let m = Mesh::new(tri.unwrap(), points, vec2(w, h));
        console::time_end_with_label("mesh new");
        m
    }

    fn new(tri: delaunator::Triangulation, points: Vec<Vec2>, dim: Vec2) -> Self {
        use delaunator::{Triangulation, EMPTY};
        let window = window().unwrap();
        let perf = window.performance().expect("performance should be enabled");

        log!("points len: {}, triangles.len {}", points.len(), tri.triangles.len());
        let n = tri.triangles.len() / 3; // number of triangles
        let mut mesh = Mesh {
            halfedges: tri.halfedges,
            triangles: tri.triangles,
            hull: tri.hull,
            points,
            adj: vec![vec![]; n],
            ids: Vec::with_capacity(n),
            dim,
            inv_ids: VecMap::with_capacity(n),
        };


        console::time_with_label("good triangles");
        // calculate 'good' triangles
        for i in 0..n {
            if mesh.valid_tri(i) {
                mesh.ids.push(i);
                mesh.inv_ids.insert(mesh.ids.len() - 1, i);
            }
        }
        console::time_end_with_label("good triangles");

        log!("after validation");

        console::time_with_label("adj time");
        /*
        let mut btreelookup = 0.;
        let mut contains = 0.;
        */
        // calculate adjacent triangles
        for (i, &e0) in mesh.halfedges.iter().enumerate() {
            // let s = perf.now();
            let _t0 = Mesh::tri_of_edge(e0);
            if !mesh.inv_ids.contains_key(_t0) { continue; }
            let &t0 = mesh.inv_ids.get(_t0).unwrap();
            // btreelookup += perf.now() - s;

            let e1 = mesh.halfedges[e0];
            if e1 == delaunator::EMPTY { continue; }

            // let s = perf.now();
            let _t1 = Mesh::tri_of_edge(e1);
            if !mesh.inv_ids.contains_key(_t1) { continue; }
            let &t1 = mesh.inv_ids.get(_t1).unwrap();
            // btreelookup += perf.now() - s;

            // let a = perf.now();
            let mut adj = &mut mesh.adj;
            if !adj[t0].contains(&t1) {
                adj[t0].push(t1)
            }

            if !adj[t1].contains(&t0) {
                adj[t1].push(t0)
            }
            // contains += perf.now() - a;
        }
        // log!("btreelookup {}, contains {}", btreelookup, contains);
        console::time_end_with_label("adj time");

        mesh
    }

    pub fn map(&self, f: fn(mesh: &Mesh, i: usize) -> f32) -> Vec<f32> {
        (0..self.ids.len())
            .map(|i| f(self, i))
            .collect()
    }

    pub fn _point_idxs_of_tri(&self, t: usize) -> [usize; 3] {
        let mut ret = [0; 3];
        for (i, &e) in Mesh::edges_of_tri(t).iter().enumerate() {
            ret[i] = self.triangles[e]
        }
        ret
    }


    pub fn _tri_path(&self, t: usize) -> [Vec2; 3] {
        let [i1, i2, i3] = self._point_idxs_of_tri(t);
        [self.points[i1], self.points[i2], self.points[i3]]
    }

    pub fn edges_of_tri(t: usize) -> [usize; 3] {
        let i = t * 3;
        [i, i + 1, i + 2]
    }

    pub fn tri_of_edge(e: usize) -> usize {
        e / 3
    }

    pub fn export_2_js(&self) -> JsValue {
        let mut obj = js_sys::Object::new();
        Reflect::define_property(&obj, "halfedges".into(), export_usize_arr2(self.halfedges.clone()));
        Reflect::define_property(&obj, "".into(), export_usize_arr2(self.halfedges.clone()));
        Reflect::define_property(&obj, "halfedges".into(), export_usize_arr2(self.halfedges.clone()));

        obj.into()

    }

    fn valid_tri(&self, i: usize) -> bool {
        if i >= self.points.len() {
            return false;
        }

        let path = self._tri_path(i);

        let a2 = path[0].magnitude_squared();
        let b2 = path[1].magnitude_squared();
        let c2 = path[2].magnitude_squared();

        let al = a2.sqrt();
        let bl = b2.sqrt();
        let cl = c2.sqrt();

        let alpha = ((b2 + c2 - a2) / (2. * bl * cl)).acos();
        let beta = ((a2 + c2 - b2) / (2. * al * cl)).acos();
        let gamma = ((a2 + b2 - c2) / (2. * al * bl)).acos();

        let ang_cutoff: f32 = 3.14 * 0.7;
        if alpha < ang_cutoff && beta < ang_cutoff && gamma < ang_cutoff {
            true
        } else {
            false
        }
    }
}
