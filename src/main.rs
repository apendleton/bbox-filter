extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate rayon;
use rayon::prelude::*;

extern crate itertools;
use itertools::Itertools;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use std::env;

#[derive(Deserialize)]
struct Properties {
    #[serde(rename = "carmen:center")]
    center: (f64, f64)
}

#[derive(Deserialize)]
struct Feature {
    properties: Properties,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args[1].clone();
    let bbox: [f64; 4] = serde_json::from_str(&args[2]).unwrap();

    let f = File::open(&filename).unwrap();
    let file = BufReader::new(&f);
    let lines = file.lines();
    for chunk in lines.chunks(10000).into_iter() {
        let v: Vec<String> = chunk.map(|l| l.unwrap()).collect();

        let filtered: Vec<&String> = v.par_iter().filter(|line| {
            let feature_res: Result<Feature, _> = serde_json::from_str(&line);
            match feature_res {
                Ok(feature) => contains(&bbox, &feature.properties.center),
                _ => false
            }
        }).collect();
        for line in filtered {
            println!("{}", line);
        }
    }
}

fn contains(bbox: &[f64; 4], pt: &(f64, f64)) -> bool {
    !(pt.0 < bbox[0] ||
        pt.0 > bbox[2] ||
        pt.1 < bbox[1] ||
        pt.1 > bbox[3])
}
