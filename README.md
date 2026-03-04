# obj-loader-rs

Archive of an old Rust OBJ parsing experiment.

## What this code does

- Uses `tobj` to load an OBJ file from disk (`assets/duck.obj`)
- Walks mesh indices and converts each face into a `PrimitiveTri` (`glam::Vec3`)
- Prints model/material counts and then prints all generated triangles

This is a prototype parser path, not a full-featured OBJ loader implementation.

## When this was worked on

From the original project (`tryobjparse`) file timestamps:

- `src/main.rs` created: December 11, 2023 at 10:18 PM (US/Central)
- `src/main.rs` last modified: December 12, 2023 at 12:09 AM (US/Central)

The original repo had no commits, so there is no commit history for finer-grained timing.

## Assets

Sample OBJ files moved under [`assets/`](assets/):

- `assets/duck.obj`
- `assets/house.obj`
- `assets/teapot.obj`
