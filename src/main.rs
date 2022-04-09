use std::io::{self, BufReader};
use flate2::bufread::GzDecoder;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("/home/pi/2022_place_canvas_history.csv.gzip")?;
    let f = BufReader::new(f);

    let mut gz = BufReader::new(GzDecoder::new(f));

    for line in gz.lines() {
        let line = line.unwrap();
        let mut it = line.split(",");
        let timestamp = it.next().unwrap();
        println!("{}", timestamp);
    }

    Ok(())
}
