extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

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
    println!("{:?}", args);

    let filename = args[1].clone();
    let bbox: [f64; 4] = serde_json::from_str(&args[2]).unwrap();

    println!("{} {:?}", filename, bbox);

    let f = File::open(&filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let line = line.unwrap();
        let feature: Feature = serde_json::from_str(&line).unwrap();
        if contains(&bbox, &feature.properties.center) {
            println!("{}", &line);
        }
    }
}

fn contains(bbox: &[f64; 4], pt: &(f64, f64)) -> bool {
    !(pt.0 < bbox[0] ||
        pt.0 > bbox[2] ||
        pt.1 < bbox[1] ||
        pt.1 > bbox[3])
}
