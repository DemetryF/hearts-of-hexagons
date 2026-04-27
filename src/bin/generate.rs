use {
    hoi25::hexagon_pos::HexagonPos,
    image::{GenericImageView, ImageReader},
    serde::Serialize,
    std::{
        collections::{HashMap, HashSet},
        env, fs,
    },
};

fn main() {
    let mut args = env::args();

    args.next();

    let picture_file = args.next().unwrap();
    let dst_file = args.next().unwrap();

    let hexagons_height = args.next().unwrap().parse().unwrap();

    let image = ImageReader::open(picture_file).unwrap().decode().unwrap();

    let side = (image.height() as f32) / (hexagons_height as f32) / 3f32.sqrt();
    let height = side * 3f32.sqrt();
    let width = 2. * side;

    let mut colors = HashSet::new();
    let mut hexagons = HashMap::new();

    for y in 0..hexagons_height {
        for x in 0.. {
            let hpos = HexagonPos::new(x, y);
            let pos = hpos.real_scaled(width, height, side);

            if pos.x > image.width() as f32 {
                break;
            }

            let color = image.get_pixel(pos.x as u32, pos.y as u32);

            if let [0, 0, 0, _] = color.0 {
                continue;
            }

            colors.insert(color.0);
            hexagons.insert(format!("{},{}", hpos.x, hpos.y), color.0);
        }
    }

    #[derive(Serialize)]
    struct Data {
        hexagons: HashMap<String, [u8; 4]>,
        colors: HashSet<[u8; 4]>,
    }

    let data = Data { hexagons, colors };

    let json = serde_json::ser::to_string(&data).unwrap();

    fs::write(dst_file, json).unwrap();
}
