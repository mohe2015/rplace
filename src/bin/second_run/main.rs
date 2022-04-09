use std::{
    fs::File,
    io::{self, BufReader},
};

use rplace::{read_rplacepixel, RPlacePixel};
use rstar::{RTree, AABB};

fn main() -> io::Result<()> {
    let mut bf = BufReader::new(File::open("test.bin")?);

    println!("Start loading points");

    let mut result: Vec<RPlacePixel> = Vec::with_capacity(160353120);
    while let Ok(v) = read_rplacepixel(&mut bf) {
        result.push(v);
    }

    println!("Done loading points. Start sorting points.");

    result.sort_unstable_by_key(|v| {
        (
            v.data.timestamp_days,
            v.data.timestamp_hours,
            v.data.timestamp_minutes,
            v.data.timestamp_seconds,
            v.data.timestamp_millis,
        )
    });

    println!("Done loading points. Start printing a point.");

    println!("{:?}", &result[0..100]);

    println!("Done printing points. Start building the r-tree.");

    let tree = RTree::bulk_load(result);

    let square = AABB::from_corners([1, 2], [3, 4]);

    println!("Found {} there", tree.locate_in_envelope(&square).count());

    println!("Done building the rtree.");

    Ok(())
}
