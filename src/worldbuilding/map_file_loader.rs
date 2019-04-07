use fnv::FnvHashMap;
use specs::prelude::*;
use fnv::FnvHashSet;
use ord_subset::*;
use vec_map::VecMap;
use std::collections::BinaryHeap;
use failure::{Error, Fail};
use std::{
    path::{Path, PathBuf},
    io::BufReader,
    fs,
    fs::File,
    iter::FromIterator,
    collections::VecDeque,
};
use super::mesh::{MeshJson, Mesh, MeshWrapper};

/// move terrain files from downloads to a special directory within project later
pub fn move_map_files() -> Result<(), Error> {
    let files = fs::read_dir("/Users/jh/Downloads")?;
    let map_files = files
        .filter_map(Result::ok)
        .filter(|d| if let Some(e) = d.path().extension() { e == "json" } else { false })
        .filter(|d: &fs::DirEntry| d.path().to_str().unwrap().contains("map_"));


    let dir_s = std::env::var("CARGO_MANIFEST_DIR")? + "/maps/ex.json";
    let dir = Path::new(&dir_s);
//    let dir = Path::new("/Users/jh/Desktop/projects/rust-proj/async/ws-rs-ex/maps/ex.json");
    for file in map_files {
        let fname = file.path();
        let s = fname.to_str().unwrap();
        fs::rename(file.path(),
                   dir.with_file_name(file.path()
                       .file_name()
                       .unwrap()))
            .map_err(|e| e.context(s.to_string()))?;
    }

    Ok(())
}

/// Loads most recent map.json file from (dir)/maps/
pub fn load_map_file(path: Option<&str>) -> Result<(Mesh, MeshJson), Error> {
    let dir = std::env::var("CARGO_MANIFEST_DIR")? + "/maps";
    debug!("{}", dir);
    let files = fs::read_dir(path.unwrap_or(&dir))?;
    let map_file = files
        .filter_map(Result::ok)
        .filter(|d| if let Some(e) = d.path().extension() { e == "json" } else { false })
        .filter(|d| d.path().to_str().unwrap().contains("map_"))
        .min_by(|a, b| {
            let f: fn(&fs::DirEntry) -> Result<f64, Error> = |x: &fs::DirEntry| Ok(fs::metadata(x.path())?.modified()?.elapsed()?.as_secs_f64());
            let a = f(a).expect("couldn't open metadata");
            let b = f(b).expect("couldn't open metadata");
            a.partial_cmp(&b).unwrap()
        });

    info!("Map File: {:?}", map_file);
    let map_file = map_file.ok_or(failure::err_msg("no file"))?.path();
    let buf_reader = BufReader::new(File::open(map_file)?);

    {
        let MeshWrapper { Mesh: mesh_json } = serde_json::from_reader(buf_reader)?;
        let clone = mesh_json.clone();
        Ok((mesh_json.into(), clone))
    }
}

