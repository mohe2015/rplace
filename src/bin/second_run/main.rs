use std::{io, fs::File};

use rplace::RPlacePixel;
use rstar::{RTree, primitives::GeomWithData};

fn main() -> io::Result<()> {
    let bf = File::open("test.bin")?;

    let mut result: Vec<GeomWithData<[i16; 2], RPlacePixel>> = bincode::deserialize_from(bf).unwrap();

    result.sort_unstable_by_key(|v| (v.timestamp_days, v.timestamp_hours, v.timestamp_minutes, v.timestamp_seconds, v.timestamp_millis));    

    println!("{:?}", result);

    let mut tree = RTree::bulk_load(result);

    Ok(())
}