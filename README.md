Map dumper for Star Ocean: Blue Sphere (GBC).

### Instructions

```sh
git clone https://github.com/scurest/blue-sphere.git
cd blue-sphere

# Copy your ROM file to ./sobs.gbc
cp ~/your/roms/StarOceanBlueSphere.gbc sobs.gbc

# Run, output files placed in ./output directory
cargo run --release --bin dump-tilesets
cargo run --release --bin dump-maps
cargo run --release --bin dump-zones
```
