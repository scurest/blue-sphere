use std::fs;
use blue_sphere::*;

fn main() {
    let rom = load_rom();

    let maps_per_row = 40;

    let mut canvas = Canvas::new(
        20 * 8 * maps_per_row,
        18 * 8 * ((NUM_MAPS as u32 + maps_per_row - 1) / maps_per_row),
    );

    for i in 0..NUM_MAPS {
        let map = Map::with_id(i);

        let x = (i as u32 % maps_per_row) * 20 * 8;
        let y = (i as u32 / maps_per_row) * 18 * 8;
        map.draw(&rom, &mut canvas, x, y);
    }

    let _ = fs::create_dir("output");
    canvas.save_png("output/maps.png");
}
