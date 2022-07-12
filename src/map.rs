/// Maps.
///
/// A map is one screen of data. Maps are identified by an ID. There are
/// about 1600 individual maps.
///
/// Map data is stored in structs (0x14 bytes each) in banks 0x54/0x55.
/// The first byte is the tileset ID for that map. The next 2*4 bytes
/// are the IDs of its neightboring maps in the up/right/down/left
/// directions. Rest of the struct is unknown.
///
/// The location of the BG data (tiles & attributes) for a map is stored
/// in a separate array in bank 0x5A. Tile & attribute data is
/// compressed with a simple RLE method

use crate::*;

pub const NUM_MAPS: u16 = 1545;

#[derive(Copy, Clone)]
pub struct Map {
    pub map_id: u16,
}

impl Map {
    pub fn with_id(map_id: u16) -> Map {
        Map { map_id }
    }

    /// Gets the map struct located in banks 0x54/0x55.
    pub fn get_map_struct(self, rom: &[u8]) -> &[u8] {
        let bank;
        let addr;

        if self.map_id < 800 {
            bank = 0x54;
            addr = 0x4001 + 0x14 * self.map_id;
        } else {
            bank = 0x55;
            addr = 0x4001 + 0x14 * (self.map_id - 800);
        }

        let offset = rom1_offset(bank, addr);
        &rom[offset..offset + 0x14]

    }

    /// Gets the tileset for this map.
    pub fn tileset_id(self, rom: &[u8]) -> u8 {
        let map_struct = self.get_map_struct(rom);
        map_struct[0]
    }

    /// Gets the neighbors of this map in the up/right/down/left
    /// directions.
    pub fn neighbors(self, rom: &[u8]) -> [Option<Map>; 4] {
        let map_struct = self.get_map_struct(rom);
        let mut neighbors = [None; 4];

        for i in 0..4 {
            let n = read_u16(map_struct, 1 + 2*i);
            let n = n & 0x7FFF;  // not sure what the high bit means...

            if n < NUM_MAPS {
                neighbors[i] = Some(Map::with_id(n));
            }
        }

        neighbors
    }

    /// Decodes BG tile & attribute data into output buffers.
    pub fn decode_bg(self, rom: &[u8], tiles: &mut [u8], attribs: &mut [u8]) {
        // Location of the compressed tile/attribute data is stored in
        // an array at $5A:4001. Three bytes per map (address + bank).
        let offset = rom1_offset(0x5A, 0x4001 + 3 * self.map_id);
        let addr = read_u16(rom, offset);
        let bank = rom[offset + 2];

        let mut decoder = BGDecoder { bank, addr };
        decoder.decode_map_tiles(rom, tiles);
        decoder.decode_map_attribs(rom, attribs);
    }

    /// Draws map on canvas with upper-left at (X,Y).
    pub fn draw(self, rom: &[u8], canvas: &mut Canvas, x: u32, y: u32) {
        let tileset_id = self.tileset_id(rom);
        let (tiles, palettes) = get_tileset(rom, tileset_id);

        let mut tile_nums = [0; 20*18];
        let mut attribs = [0; 25*18];    // sometimes need more than 20*18 bytes?

        self.decode_bg(rom, &mut tile_nums, &mut attribs);

        for i in 0..20*18 {
            let tile_num = tile_nums[i];
            let palette_num = attribs[i] & 7;

            // Get the address of the tile in VRAM for this tile number.
            // Maps use tile addressing mode 1 (bit 4 of LCDC register).
            let tile_num = tile_num as i8 as i32;
            let addr = (0x9000 + tile_num * 16) as u16;

            // Address should be in the the tileset region $8C00..$9800.
            assert!((0x8C00..0x9800).contains(&addr));

            let ofs = addr as usize - 0x8C00;
            let tile = &tiles[ofs..ofs + 16];

            let ofs = 8 * palette_num as usize;
            let palette = &palettes[ofs..ofs + 8];

            let dx = (i as u32 % 20) * 8;
            let dy = (i as u32 / 20) * 8;

            canvas.draw_tile(tile, palette, x + dx, y + dy);
        }
    }
}

struct BGDecoder {
    bank: u8,
    addr: u16,
}

impl BGDecoder {
    /// Decodes compressed BG map tile numbers into output.
    fn decode_map_tiles(&mut self, rom: &[u8], output: &mut [u8]) {
        // Coresponding subroutine: $0B28
        // Decodes map tile data at de into the region $D85D..$D9C5.

        assert!((0x4000..0x8000).contains(&self.addr), "address not in ROM1");

        let offset = rom1_offset(self.bank, self.addr);
        let input = &rom[offset..];

        let mut i = 0;
        let mut j = 0;

        while j < output.len() {
            let a = input[i];
            i += 1;

            if a & 0x80 == 0 || a >= 0xA0 {

                // Copy literal byte to output.
                output[j] = a;
                j += 1;

            } else if a & 0x10 != 0 {

                // Copy the previous byte count times, incrementing by 1 each
                // time.
                let count = (a & 0xF) + 2;
                for _ in 0..count {
                    output[j] = output[j - 1].wrapping_add(1);
                    j += 1;
                }

            } else {

                // Copy the previous byte count times.
                let count = (a & 0xF) + 2;
                for _ in 0..count {
                    output[j] = output[j - 1];
                    j += 1;
                }

            }
        }

        // Move addr to the end of the tile data. Attribute data comes next.
        assert!(self.addr as usize + i < 0x8000, "addr moved out of ROM1");
        self.addr += i as u16;
    }

    /// Decodes compressed BG map attributes into output.
    fn decode_map_attribs(&mut self, rom: &[u8], output: &mut [u8]) {
        // Coresponding subroutine: $0BAD
        // Decodes map attribute data at de into the region $D9C5..$DB2D.

        let offset = rom1_offset(self.bank, self.addr);
        let mut input = &rom[offset..];

        // If the first byte is 7, the attributes don't fit in the
        // current bank. Move up to the next one.
        if input[0] == 7 {
            self.bank += 1;
            self.addr = 0x4001;
            let offset = rom1_offset(self.bank, self.addr);
            input = &rom[offset..];
        }

        let mut i = 0;
        let mut j = 0;

        while j < 20 * 19 {
            let a = input[i];
            i += 1;

            if a & 0x6 != 0x6 {

                // Copy literal byte to output.
                //
                // Note that a & 0x6 != 0x6 is equivalent to a & 7 being
                // neither 6 nor 7. Since a & 7 is also the palette
                // number, this means maps never use palette 6 or 7.
                output[j] = a;
                j += 1;

            } else {

                // Copy the previous byte count times.
                let count = ((a >> 4) | ((a & 1) << 4) ) + 2;
                for _ in 0..count {
                    output[j] = output[j - 1];
                    j += 1;
                }

            }
        }
    }
}
