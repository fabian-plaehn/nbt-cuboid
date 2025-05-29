# nbt-quadrant

A simple Rust CLI tool to generate Minecraft structure NBT files filled with a single block type in a specified cuboid region.

## Features
- Prompts for cuboid dimensions (X, Y, Z) and block type
- Generates a structure NBT file with all blocks set to the chosen type
- Output is compressed in GZip NBT format, ready for Minecraft

## Usage
1. Build the project:
   ```powershell
   cargo build --release
   ```
2. Run the executable and follow the prompts:
   ```powershell
   target\release\nbt-cuboid.exe
   ```
3. Enter the desired size and block name (e.g., `minecraft:stone`).
4. The generated NBT file will be saved in the current directory.