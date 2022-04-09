use std::io::{self, BufReader};
use flate2::bufread::GzDecoder;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let f = File::open("/home/pi/2022_place_canvas_history.csv.gzip")?;
    let f = BufReader::new(f);

    let gz = BufReader::new(GzDecoder::new(f));

    let mut next_user_id = -1;
    let mut next_pixel_color = -1;
    let mut user_ids = HashMap::new();
    let mut pixel_colors = HashMap::new();
    for line in gz.lines().skip(1) {
        let line = line.unwrap();
        let mut it = line.split(',');
        let timestamp = it.next().unwrap();
        let user_id = it.next().unwrap();
        let user_id = user_ids.entry(user_id.to_string()).or_insert_with(|| { next_user_id += 1; next_user_id });
        let pixel_color = &it.next().unwrap()[1..];
        let pixel_color = pixel_colors.entry(pixel_color.to_string()).or_insert_with(|| { next_pixel_color += 1; next_pixel_color });
        let coordinate_x = &it.next().unwrap()[1..];
        let coordinate_y = it.next().unwrap();
        let coordinate_y = &coordinate_y[..coordinate_y.len()-1];
        println!("{},{},{},{},{}", timestamp, user_id, pixel_color, coordinate_x, coordinate_y);
    }

    eprintln!("{:#?}", pixel_colors);
    eprintln!("user count: {}", next_user_id);

    Ok(())
}
