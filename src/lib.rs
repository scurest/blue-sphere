pub mod canvas;
pub mod map;
pub mod tileset;

pub use canvas::*;
pub use map::*;
pub use tileset::*;

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
