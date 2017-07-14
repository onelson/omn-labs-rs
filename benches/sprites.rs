#![feature(test)]

extern crate test;
extern crate omn_labs;

use omn_labs::sprites::{ClipStore, SpriteSheetData, PlayMode};


#[bench]
fn create_clip_store(b: &mut test::Bencher) {
    let sheet = SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json");
    b.iter(|| ClipStore::new(&sheet));
}

#[bench]
fn create_clip_instance(b: &mut test::Bencher) {
    let sheet = SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json");
    let clips = ClipStore::new(&sheet);
    b.iter(|| clips.create("Alpha", PlayMode::Loop).unwrap());
}

#[bench]
fn clip_update(b: &mut test::Bencher) {
    let sheet = SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json");
    let clips = ClipStore::new(&sheet);
    let mut clip = clips.create("Alpha", PlayMode::Loop).unwrap();
    b.iter(|| clip.update(800.));
}

#[bench]
fn clip_get_cell(b: &mut test::Bencher) {
    let sheet = SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json");
    let clips = ClipStore::new(&sheet);
    let mut clip = clips.create("Alpha", PlayMode::Loop).unwrap();
    b.iter(|| {
        clip.update(800.);
        clip.get_cell()
    });
}
