#![feature(test)]

extern crate test;

extern crate omn_labs;

use omn_labs::sprites::{SpriteSheetData, PlayMode};

#[bench]
fn load_from_file(b: &mut test::Bencher) {
    b.iter(|| SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json"));
}

#[bench]
fn create_clip_instance(b: &mut test::Bencher) {
    let sheet = SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json");
    b.iter(|| sheet.clips.create("Alpha", PlayMode::Loop).unwrap());
}

#[bench]
fn clip_update(b: &mut test::Bencher) {
    let sheet = SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json");
    let mut clip = sheet.clips.create("Alpha", PlayMode::Loop).unwrap();
    b.iter(|| clip.update(800.));
}
