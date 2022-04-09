use std::io::{self, BufReader};
use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::fs::File;
use rustc_hash::FxHashMap;
use base64::{decode_config_slice, STANDARD};

// cat /home/pi/2022_place_canvas_history.csv.gzip | gunzip | wc -c
// 21714634193
// ~22 GB

// 1m46.864s

// hex parsing: 

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let f = File::open("/home/pi/2022_place_canvas_history.csv.gzip")?;
    let mut gz = BufReader::new(GzDecoder::new(f));

    let mut next_user_id = -1;
    //let mut next_pixel_color = -1;
    let mut user_ids = FxHashMap::default();
    //let mut pixel_colors = FxHashMap::default();
    let mut line = Vec::<u8>::new();
    let mut number_of_bytes_read = 0;
    gz.read_until(b'\n', &mut line)?; // drop header
    line.clear();
    loop {
        let line_length = gz.read_until(b'\n', &mut line)?;
        if line_length == 0 {
            break;
        }
        number_of_bytes_read += line_length;
        if number_of_bytes_read % (256 * 1024) == 0 {
            eprintln!("{} %", f64::from(u32::try_from(number_of_bytes_read).unwrap()) / 21714634193f64);
        }

        // unfortunately already timestamps can have two digit or three digit milliseconds so we need to split the data. userid and colors should be the same though
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
        let pixel_color = u32::from_str_radix(std::str::from_utf8(&it.next().unwrap()[1..]).unwrap(), 16).unwrap();
        /*let pixel_color = match pixel_colors.get(&pixel_color) {
            Some(v) => *v,
            None => {
                next_pixel_color += 1;
                pixel_colors.insert(pixel_color, next_pixel_color);
                next_pixel_color
            }
        };*/
        let pixel_color = match pixel_color {
            52416 => 0,
            0 => 5,
            11815616 => 20,
            16729344 => 25,
            5368308 => 13,
            16754688 => 27,
            41832 => 7,
            8461983 => 17,
            13948889 => 22,
            8318294 => 16,
            9014672 => 18,
            4799169 => 11,
            16726145 => 24,
            52344 => 8,
            16766517 => 29,
            16775352 => 30,
            30063 => 6,
            7161903 => 15,
            10250534 => 19,
            7143450 => 14,
            6970623 => 2,
            9745407 => 1,
            14986239 => 4,
            12451897 => 21,
            14553215 => 23,
            16757872 => 28,
            16777215 => 31,
            40618 => 3,
            3576042 => 10,
            2379940 => 9,
            16751018 => 26,
            5329490 => 12,
            _ => panic!(),
        };
        let coordinate_x = std::str::from_utf8(&it.next().unwrap()[1..]).unwrap().parse::<u16>().unwrap();
        let coordinate_y = it.next().unwrap();
        let coordinate_y = std::str::from_utf8(&coordinate_y[..coordinate_y.len()-2]).unwrap().parse::<u16>().unwrap();
        writeln!(handle, "{},{},{},{},{}", std::str::from_utf8(timestamp).unwrap(), user_id, pixel_color, coordinate_x, coordinate_y)?;
        line.clear();
    }

    //eprintln!("{:#?}", pixel_colors);
    eprintln!("user count: {}", next_user_id);

    Ok(())
}
