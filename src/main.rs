use std::io::{self, BufReader};
use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

// rustc-hash

// almost everything in a line should be at a fixed offset so maybe cheat

// lines() probably allocates

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let f = File::open("/home/pi/2022_place_canvas_history.csv.gzip")?;

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
        // maybe base64 decode
        let user_id = match user_ids.get(user_id) {
            Some(v) => *v,
            None => {
                next_user_id += 1;
                user_ids.insert(user_id.to_string(), next_user_id);
                next_user_id
            }
        };
        let pixel_color = &it.next().unwrap()[1..];
        // maybe hex decode or store statically as the colors should be known
        let pixel_color = match pixel_colors.get(pixel_color) {
            Some(v) => *v,
            None => {
                next_pixel_color += 1;
                pixel_colors.insert(pixel_color.to_string(), next_pixel_color);
                next_pixel_color
            }
        };
        let coordinate_x = &it.next().unwrap()[1..];
        let coordinate_y = it.next().unwrap();
        let coordinate_y = &coordinate_y[..coordinate_y.len()-1];
        //write!(handle, "{},{},{},{},{}", timestamp, user_id, pixel_color, coordinate_x, coordinate_y)?;
    }

    eprintln!("{:#?}", pixel_colors);
    //eprintln!("user count: {}", next_user_id);

    Ok(())
}
