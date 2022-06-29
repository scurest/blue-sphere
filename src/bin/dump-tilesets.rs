use std::fs;
use blue_sphere::*;

fn main() {
    let rom = load_rom();

    static W: u32 = 7;
    static H: u32 = 14;
    assert!(W * H == NUM_TILESETS as u32);

    let block_w = 16 * 8 + 8;
    let block_h = 12 * 8 + 8;

    let mut canvas = Canvas::new(W * block_w - 8, H * block_h - 8);

    for tileset_id in 0..NUM_TILESETS {
        let (tiles, palettes) = get_tileset(&rom, tileset_id);

        // Use the first palette for all tiles
        let palette = &palettes[..8];

        let block_x = (tileset_id as u32 % W) * block_w;
        let block_y = (tileset_id as u32 / W) * block_h;

        for (i, tile) in tiles.chunks_exact(16).enumerate() {
            let x = (i as u32 % 16) * 8 + block_x;
            let y = (i as u32 / 16) * 8 + block_y;
            canvas.draw_tile(tile, palette, x as u32, y as u32);
        }
    }

    let _ = fs::create_dir("output");
    canvas.save_png("output/tilesets.png");
}
