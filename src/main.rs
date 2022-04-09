use std::io::{self, BufReader};
use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::fs::File;
use rustc_hash::FxHashMap;
use base64::{decode_config_slice, STANDARD};

// almost everything in a line should be at a fixed offset so maybe cheat

// cat /home/pi/2022_place_canvas_history.csv.gzip | gunzip | wc -c
// 21714634193
// ~22 GB

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let f = File::open("/home/pi/2022_place_canvas_history.csv.gzip")?;
    let mut gz = BufReader::new(GzDecoder::new(f));

    let mut next_user_id = -1;
    let mut next_pixel_color = -1;
    let mut user_ids = FxHashMap::default();
    let mut pixel_colors = FxHashMap::default();
    let mut line = Vec::<u8>::new();
    gz.read_until(b'\n', &mut line)?; // drop header
    line.clear();
    while gz.read_until(b'\n', &mut line)? != 0 {
        let mut it = line.split(|c| *c == b',');
        let timestamp = it.next().unwrap();
        let mut user_id = vec![0; 64];
        assert_eq!(64, decode_config_slice(it.next().unwrap(), STANDARD, &mut user_id).unwrap());
        let user_id = match user_ids.get(&user_id) {
            Some(v) => *v,
            None => {
                next_user_id += 1;
                user_ids.insert(user_id.clone(), next_user_id);
                next_user_id
            }
        };
        let pixel_color = &it.next().unwrap()[1..];
        // maybe hex decode or store statically as the colors should be known
        let pixel_color = match pixel_colors.get(pixel_color) {
            Some(v) => *v,
            None => {
                next_pixel_color += 1;
                pixel_colors.insert(pixel_color.to_owned(), next_pixel_color);
                next_pixel_color
            }
        };
        let coordinate_x = std::str::from_utf8(&it.next().unwrap()[1..]).unwrap().parse::<u16>().unwrap();
        let coordinate_y = it.next().unwrap();
        let coordinate_y = std::str::from_utf8(&coordinate_y[..coordinate_y.len()-2]).unwrap().parse::<u16>().unwrap();
        writeln!(handle, "{},{},{},{},{}", std::str::from_utf8(timestamp).unwrap(), user_id, pixel_color, coordinate_x, coordinate_y)?;
        line.clear();
    }

    eprintln!("{:#?}", pixel_colors);
    eprintln!("user count: {}", next_user_id);

    Ok(())
}
