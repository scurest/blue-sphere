/// Tilesets.
///
/// A tileset is a block of 16x12 tile patterns (0xC00 bytes) followed
/// by 8 palettes (0x40 bytes).
///
/// When a tileset is loaded, it's tiles are copied into VRAM region
/// $8C00..$9800 and it's palettes become the background palettes.
///
/// Tilesets are located in banks $40-$53, five per bank. There are 98
/// tilesets total. Tilesets 98 and 99 are zero filled.

pub const NUM_TILESETS: u8 = 98;

/// Get the ROM offset to a tileset.
pub fn tileset_offset(tileset_id: u8) -> usize {
    // Corresponding subroutine: $05:402B
    // Input: a = tileset_id
    // Output: a = bank number
    //         hl = address of tileset

    let bank = 0x40 + tileset_id as usize / 5;
    let addr = 0x4001 + (tileset_id as usize % 5) * 0x0C40;

    0x4000 * bank + (addr - 0x4000)
}

/// Get the tiles and palettes of a tileset from the ROM.
pub fn get_tileset(rom: &[u8], tileset_id: u8) -> (&[u8], &[u8]) {
    let ofs = tileset_offset(tileset_id);
    let tiles = &rom[ofs..ofs + 0xC00];
    let palettes = &rom[ofs + 0xC00..ofs + 0xC40];
    (tiles, palettes)
}
