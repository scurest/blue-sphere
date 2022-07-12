pub mod canvas;
pub mod map;
pub mod tileset;

pub use canvas::*;
pub use map::*;
pub use tileset::*;

pub fn read_u16(buf: &[u8], offset: usize) -> u16 {
    (buf[offset] as u16) | ((buf[offset+1] as u16) << 8)
}

/// Get the ROM offset for a ROM1 address bank:addr.
pub fn rom1_offset(bank: u8, addr: u16) -> usize {
    assert!((0x4000..0x8000).contains(&addr), "address not in ROM1");
    0x4000 * bank as usize + (addr as usize - 0x4000)
}

/// Loads ROM file from current fold1er.
pub fn load_rom() -> Vec<u8> {
    use std::fs;

    let path = "sobs.gbc";

    match fs::read(path) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("need a sobs.gbc romfile in the current folder");
            eprintln!("exiting");
            std::process::exit(1);
        }
    }
}
