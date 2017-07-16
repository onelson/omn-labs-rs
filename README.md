# Omn Labs [![Build Status](https://api.travis-ci.org/onelson/omn-labs-rs.svg?branch=master)](https://travis-ci.org/onelson/omn-labs-rs)

A little incubator crate for the glimmer of ideas and crates yet to be.

## Development Preamble

Moving to sdl2 for the foundation of this engine code, and as such you'll 
need to satisfy some bits from the crates install docs, specifically, the section on the SDL 
devel files <https://github.com/AngryLawyer/rust-sdl2#sdl20-development-libraries>

OmnLabs is, in theory, a library of reusable "game stuff" built on top of [ggez]. Anything deemed reusable should be 
under `src/` and prototypes etc should be coded as standalone programs under `examples/`.  Examples can be run via 
`cargo run --example <name>` where name is the name of the module (eg `cargo run --example spinner`). 

[ggez]: https://github.com/ggez/ggez
