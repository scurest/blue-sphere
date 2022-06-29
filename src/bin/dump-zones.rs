// Dump images for all zones in the game. A zone is a contiguous set
// of map, ie. a map, plus all its neighbors, plus all their neighbors,
// etc. Each map is placed in exactly one zone.

use std::fs;
use std::collections::HashMap;
use blue_sphere::*;

struct Zone {
    map_locations: HashMap<(i16, i16), u16>,

    /// Tracks maps that have already been placed.
    already_visited: Vec<bool>,
}

impl Zone {
    fn new() -> Zone {
        Zone {
            map_locations: HashMap::new(),
            already_visited: vec![false; NUM_MAPS as usize],
        }
    }

    fn build_zone(&mut self, rom: &[u8], starting_map: u16) {
        self.map_locations.clear();

        // Seed to grow the zone from.
        struct Seed {
            map_id: u16,
            location: (i16, i16),
        }

        let first_seed = Seed {
            map_id: starting_map,
            location: (0, 0),
        };

        let mut seeds = vec![first_seed];

        // XY deltas for the four neighbors.
        let deltas = [
            ( 0, -1),  // up
            ( 1,  0),  // right
            ( 0,  1),  // down
            (-1,  0),  // left
        ];

        while let Some(seed) = seeds.pop() {
            self.map_locations.insert(seed.location, seed.map_id);
            self.already_visited[seed.map_id as usize] = true;

            let map = Map::with_id(seed.map_id);
            let neighbors = map.neighbors(rom);

            // Seed with the four neighbors.
            for i in 0..4 {
                if let Some(neighbor) = neighbors[i] {
                    let (x, y) = seed.location;
                    let (dx, dy) = deltas[i];
                    let location = (x + dx, y + dy);

                    if self.already_visited[neighbor.map_id as usize] {
                        // Skip maps that are already placed.
                        // NOTE: Zones might loop or have discontinuities.
                        continue;
                    }

                    let new_seed = Seed {
                        map_id: neighbor.map_id,
                        location,
                    };
                    seeds.push(new_seed);
                }
            }
        }
    }

    fn draw_zone(&self, rom: &[u8]) -> Canvas {
        // Find zone bounds.
        let x_min = self.map_locations.keys().map(|k| k.0).min().unwrap();
        let x_max = self.map_locations.keys().map(|k| k.0).max().unwrap();
        let y_min = self.map_locations.keys().map(|k| k.1).min().unwrap();
        let y_max = self.map_locations.keys().map(|k| k.1).max().unwrap();

        let w = (x_max - x_min + 1) as u32;
        let h = (y_max - y_min + 1) as u32;

        let mut canvas = Canvas::new(20 * 8 * w, 18 * 8 * h);

        for ((x, y), &map_id) in self.map_locations.iter() {
            let map = Map::with_id(map_id);
            map.draw(
                rom,
                &mut canvas,
                20 * 8 * (x - x_min) as u32,
                18 * 8 * (y - y_min) as u32,
            );
        }

        canvas
    }
}

fn main() {
    let rom = load_rom();
    let _ = fs::create_dir("output");

    let mut zone = Zone::new();
    let mut zone_i = 0;

    for map_id in 0..NUM_MAPS {

        // Map was already in a previous zone, skip it.
        if zone.already_visited[map_id as usize] {
            continue;
        }

        zone.build_zone(&rom, map_id);

        let canvas = zone.draw_zone(&rom);
        let path = format!("output/zone{:03}.png", zone_i);
        canvas.save_png(&path);
        zone_i += 1;

    }
}
