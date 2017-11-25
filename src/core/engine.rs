use std::path::{Path, PathBuf};
use std::fs;

pub struct Engine {
    dynamics_enabled: bool,

    pub asset_path: PathBuf
}

impl Engine {
    pub fn new(asset_path: PathBuf) -> Engine {
        let mut canonical_path: PathBuf = PathBuf::from("./");

        match fs::canonicalize(asset_path) {
            Ok(val) => canonical_path = val,
            Err(err) => println!("{:?}", err)
        }

        Engine {
            dynamics_enabled: true,
            asset_path: canonical_path
        }
    }

    pub fn start(&self) {
        println!("Asset path: {0}", self.asset_path.to_str().unwrap());
    }

    pub fn step(&self, dt: f64) {

    }
}