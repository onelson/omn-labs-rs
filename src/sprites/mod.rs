//! The `sprites` module contains types and functions for managing playback of frame sequences
//! over time.

use std::path::Path;
use std::fs::File;
use std::collections::hash_map::HashMap;
use serde_json;

mod aseprite;


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Region {
    pub x: i32,
    pub y: i32,
    #[serde(rename="w")]
    pub width: i32,
    #[serde(rename="h")]
    pub height: i32
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Frame {
    pub duration: i32,
    #[serde(rename="frame")]
    pub bbox: Region,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Forward,
    Reverse,
    PingPong
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FrameTag {
    pub name: String,
    pub from: usize,
    pub to: usize,
    // one of "forward", "reverse", "pingpong"
    pub direction: String
}

pub type Delta = f32;
pub type FrameDuration = i32;

#[derive(Debug, Clone)]
pub struct CellInfo {
    pub idx: usize,
    pub duration: FrameDuration
}


/// AnimationClip is a group of cell indexes paired with durations such that it can track
/// playback progress over time. It answers the question of "what subsection of a sprite sheet
/// should I render at this time?"
///
/// # Examples
///
/// ```
/// use omn_labs::sprites::{AnimationClip, Delta, Frame, Region, Direction};
///
/// let frames = vec![
///     Frame { duration: 1000, bbox: Region { x: 0, y: 0, width: 32, height: 32 } },
///     Frame { duration: 1000, bbox: Region { x: 32, y: 0, width: 32, height: 32 } },
/// ];
///
/// let mut clip = AnimationClip::new(
///     &frames,
///     Direction::Forward
/// );
///
/// assert_eq!(clip.get_cell(), 0);
/// clip.update(800.);
///
/// assert_eq!(clip.get_cell(), 0);
/// clip.update(800.);
///
/// // as playback progresses, we get different frames as a return
/// assert_eq!(clip.get_cell(), 1);
/// clip.update(800.);
///
/// // and as the "play head" extends beyond the total duration of the clip, it'll loop back
/// // around to the start. This wrapping behaviour can be customized via the `Direction` parameter.
/// assert_eq!(clip.get_cell(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct AnimationClip {
    current_time: Delta,  // represents the "play head"
    pub direction: Direction,
    pub duration: Delta,
    // FIXME: should be a vec of durations. Should pair with a separate object with the frame data?
    // The same frames will likely be part of other clips. Could simply index into an object
    // representing the full sprite sheet. If we support "direction" as a playback option, we'll
    // need something to manage the mapping of indices, especially wrt "ping-pong".
    cells: Vec<CellInfo>
}


impl AnimationClip {
    pub fn new<'a>(frames: &'a [Frame], direction: Direction) -> Self {

        let cell_info: Vec<CellInfo> = match direction {
            Direction::Forward =>
                frames.iter().enumerate()
                    .map(|(idx, ref x)| CellInfo { idx: idx, duration: x.duration})
                    .collect(),
            Direction::Reverse =>
                frames.iter().enumerate().rev()
                    .map(|(idx, ref x)| CellInfo { idx: idx, duration: x.duration})
                    .collect(),
            // Look at what aseprite does about each end (double frame problem)
            Direction::PingPong =>
                frames.iter().enumerate().chain(frames.iter().enumerate().rev())
                    .map(|(idx, ref x)| CellInfo { idx: idx, duration: x.duration})
                    .collect(),

        };

        AnimationClip {
            current_time: 0.,
            direction: direction,
            duration: cell_info.iter().map(|x| x.duration as Delta).sum(),
            cells: cell_info
        }
    }

    pub fn update(&mut self, dt: Delta) {
        self.current_time = (self.current_time + dt) % self.duration;
    }

    pub fn set_time(&mut self, time: Delta) {
        self.current_time = time % self.duration;
    }

    pub fn reset(&mut self) {
        self.set_time(0.);
    }

    #[allow(dead_code)]
    pub fn get_cell(&self) -> usize {
        let mut remaining_time = self.current_time;
        for cell in self.cells.iter().cycle() {
            remaining_time -= cell.duration as Delta;
            if remaining_time <= 0. { return cell.idx; }
        }
        unreachable!();
    }
}

pub struct ClipStore {
    clips: HashMap<String, AnimationClip>
}

impl ClipStore {
    // FIXME: rename method - `get()` looks like hashmap access, but this is generating a new clone!
    pub fn get(&self, key: &str) -> Option<AnimationClip> {
        self.clips.get(key).map(|x| x.clone())
    }
}

pub struct SpriteSheetData {
    pub cells: Vec<Frame>,
    pub clips: ClipStore
}

impl SpriteSheetData {

    pub fn from_json_str(json: &str) -> Self {
        let data: aseprite::ExportData = serde_json::from_str(json).unwrap();
        SpriteSheetData::from_aesprite_data(data)
    }

    pub fn from_json_value(json: serde_json::Value) -> Self {
        let data: aseprite::ExportData = serde_json::from_value(json).unwrap();
        SpriteSheetData::from_aesprite_data(data)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let data: aseprite::ExportData = serde_json::from_reader(File::open(path).unwrap()).unwrap();
        SpriteSheetData::from_aesprite_data(data)
    }
    pub fn from_aesprite_data(data: aseprite::ExportData) -> Self {
        let mut clips = HashMap::new();

        for tag in data.meta.frame_tags {

            let direction = match tag.direction.as_ref() {
                "forward" => Direction::Forward,
                "reverse" => Direction::Reverse,
                "pingpong" => Direction::PingPong,
                _ => Direction::Forward,
            };
            let frames: &[Frame] = &data.frames[tag.from .. tag.to];
            clips.insert(tag.name, AnimationClip::new(frames, direction));
        }

        SpriteSheetData {
            cells: data.frames,
            clips: ClipStore { clips: clips }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_from_file() {
        let sheet = SpriteSheetData::from_file("examples/resources/numbers/numbers-matrix-tags.array.json");
        let alpha = sheet.clips.get("Alpha").unwrap();
        let beta = sheet.clips.get("Beta").unwrap();
        let gamma = sheet.clips.get("Gamma").unwrap();
        assert_eq!(alpha.get_cell(), 0);
        assert_eq!(beta.get_cell(), 0);
        assert_eq!(gamma.get_cell(), 0);
    }

    #[test]
    fn test_clips_are_distinct() {
        let sheet = SpriteSheetData::from_file("examples/resources/numbers/numbers-matrix-tags.array.json");

        // Each time we get a named clip, we're creating a new instance, and each have their
        // own internal clock.
        let mut alpha1 = sheet.clips.get("Alpha").unwrap();
        let mut alpha2 = sheet.clips.get("Alpha").unwrap();

        alpha1.update(20.);
        alpha2.update(120.);

        assert_eq!(alpha1.get_cell(), 0);
        assert_eq!(alpha2.get_cell(), 1);
    }
}
