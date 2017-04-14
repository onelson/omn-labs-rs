# OmnLabs [![Build Status](https://api.travis-ci.org/onelson/OmnLabsRS.svg?branch=master)](https://travis-ci.org/onelson/OmnLabsRS)

## Development Preamble

Moving to sdl2 for the foundation of this engine code, and as such you'll 
need to satisfy some bits from the crates install docs, specifically, the section on the SDL 
devel files <https://github.com/AngryLawyer/rust-sdl2#sdl20-development-libraries>

OmnLabs is, in theory, a library of reusable "game stuff" built on top of [ggez]. Anything deemed reusable should be 
under `src/` and prototypes etc should be coded as standalone programs under `examples/`.  Examples can be run via 
`cargo run --example <name>` where name is the name of the module (eg `cargo run --example spinner`). 

## Running

When running the app via `cargo run`, you have to copy or symlink the `resources` directory to the build location, 
for example `ln -s resources target/debug/`. Failing to do so will result in a panic from `assets.rs` when the module 
attempts to read a file it cannot locate. The error should contain hints for where it is expecting the files to be.  

[ggez]: https://github.com/ggez/ggez