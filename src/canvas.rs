use crate::*;

/// Canvas of RGB888 pixels.
pub struct Canvas {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Canvas {
    /// Create black canvas.
    pub fn new(w: u32, h: u32) -> Canvas {
        Canvas {
            pixels: vec![0; 3 * w as usize * h as usize],
            width: w,
            height: h,
        }
    }

    /// Put RGB color at (X,Y).
    pub fn put_rgb(&mut self, (r, g, b): (u8, u8, u8), x: u32, y: u32) {
        if x < self.width && y < self.height {
            let offset = (3 * self.width * y + 3 * x) as usize;
            self.pixels[offset + 0] = r;
            self.pixels[offset + 1] = g;
            self.pixels[offset + 2] = b;
        }
    }

    /// Put color from a GBC palette at (X,Y).
    pub fn put_color(&mut self, color_num: u8, palette: &[u8], x: u32, y: u32) {
        let bgr = read_u16(palette, 2 * color_num as usize);

        let r = (bgr >> 0) & 0x1F;
        let g = (bgr >> 5) & 0x1F;
        let b = (bgr >> 10) & 0x1F;

        // 5bpp -> 8bpp
        let r = (r * 255 + 15) / 31;
        let g = (g * 255 + 15) / 31;
        let b = (b * 255 + 15) / 31;

        // NOTE: raw color, no GBC color correction

        self.put_rgb((r as u8, g as u8, b as u8), x, y);
    }

    /// Draw GBC tile with the top-left at the (X,Y) position.
    pub fn draw_tile(&mut self, tile: &[u8], palette: &[u8], x: u32, y: u32) {
        // A tile is 8x8 pixels.
        // 16 bytes. 2 bytes = 1 row of 8 pixels.
        // A pixel is a color number 0-3 in the palette.
        // Example row:
        //
        //     [Color#] --> [Bits]   --> [Bytes]
        //     01233210 --> 01011010 --> 0x5a
        //                  00111100 --> 0x3c

        let mut i = 0;
        for dy in 0..8 {
            for dx in 0..8 {
                let mask = 1 << (7 - dx);
                let lo = (tile[i] & mask) != 0;
                let hi = (tile[i+1] & mask) != 0;
                let color_num = lo as u8 | ((hi as u8) << 1);
                self.put_color(color_num, palette, x + dx, y + dy);
            }
            i += 2;
        }
    }

    /// Save as PNG.
    pub fn save_png(&self, path: &str) {
        use std::fs::File;
        use std::io::BufWriter;

        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.pixels).unwrap();
    }
}
