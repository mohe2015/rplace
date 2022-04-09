use std::{io::{self, BufReader}, fs::File};

use rplace::{RPlacePixel, read_rplacepixel};
use rstar::{RTree, primitives::GeomWithData};

fn main() -> io::Result<()> {
    let mut bf = BufReader::new(File::open("test.bin")?);

    println!("Start loading points");

    let mut result: Vec<RPlacePixel> = Vec::new();
    loop {
        match read_rplacepixel(&mut bf) {
            Ok(v) => result.push(v),
            Err(_) => break
        }
    }

    println!("Done loading points. Start sorting points.");

    result.sort_unstable_by_key(|v| (v.data.timestamp_days, v.data.timestamp_hours, v.data.timestamp_minutes, v.data.timestamp_seconds, v.data.timestamp_millis));    

    println!("Done loading points. Start printing a point.");

    println!("{:?}", result[0]);

    println!("Done printing points. Start building the r-tree.");

    let mut tree = RTree::bulk_load(result);

    println!("Done building the rtree.");

    Ok(())
}