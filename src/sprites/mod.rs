//! The `sprites` module contains types and functions for managing playback of frame sequences
//! over time.

use std::collections::hash_map::HashMap;
pub mod aseprite;


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Region {
    pub x: i32,
    pub y: i32,
    #[serde(rename = "w")]
    pub width: i32,
    #[serde(rename = "h")]
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Frame {
    pub duration: i32,
    #[serde(rename = "frame")]
    pub bbox: Region,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Forward,
    Reverse,
    PingPong,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FrameTag {
    pub name: String,
    pub from: usize,
    pub to: usize,
    // one of "forward", "reverse", "pingpong"
    pub direction: String,
}

pub type Delta = f32;
pub type FrameDuration = i32;

/// `CellInfo.idx` points to an index in `SpriteSheetData.cells` and `CellInfo.duration` indicates
/// how long this section of the texture atlas should be displayed as per an `AnimationClip`.
#[derive(Debug, Clone)]
pub struct CellInfo {
    pub idx: usize,
    pub duration: FrameDuration,
}


/// `PlayMode` controls how the current frame data for a clip at a certain time is calculated with
/// regards to the duration bounds.
#[derive(PartialEq, Debug, Clone)]
pub enum PlayMode {
    /// `OneShot` will play start to finish, but requests for `CellInfo` after the duration will get
    /// you None.
    OneShot,
    /// `Hold` is similar to `OneShot` however time past the end of the duration will repeat
    /// the final frame.
    Hold,
    /// A `Loop` clip never ends and will return to the start of the clip when exhausted.
    Loop,
}

#[derive(Debug)]
pub struct AnimationClipTemplate {
    pub cells: Vec<CellInfo>,
    pub direction: Direction,
    pub duration: Delta,
    pub name: String,
}

impl AnimationClipTemplate {
    pub fn new(name: String, frames: &[Frame], direction: Direction, offset: usize) -> Self {
        let cell_info: Vec<CellInfo> = match direction {
            Direction::Reverse =>
                frames.iter().enumerate().rev()
                    .map(|(idx, x)| CellInfo { idx: offset + idx, duration: x.duration})
                    .collect(),
            // Look at what aseprite does about each end (double frame problem)
            Direction::PingPong =>
                frames.iter().enumerate().chain(frames.iter().enumerate().rev())
                    .map(|(idx, x)| CellInfo { idx: offset + idx, duration: x.duration})
                    .collect(),
            _ =>  // assumes Forward in the fallback case
                frames.iter().enumerate()
                    .map(|(idx, x)| CellInfo { idx: offset + idx, duration: x.duration})
                    .collect()

        };
        let duration = cell_info.iter().map(|x| x.duration as Delta).sum();
        Self {
            name: name,
            cells: cell_info,
            direction: direction,
            duration: duration,
        }
    }
}

/// `AnimationClip` is a group of cell indexes paired with durations such that it can track
/// playback progress over time. It answers the question of "what subsection of a sprite sheet
/// should I render at this time?"
///
/// # Examples
///
/// ```
/// use omn_labs::sprites::{AnimationClip, Delta, Frame, Region, Direction, PlayMode};
///
/// let frames = vec![
///     Frame { duration: 1000, bbox: Region { x: 0, y: 0, width: 32, height: 32 } },
///     Frame { duration: 1000, bbox: Region { x: 32, y: 0, width: 32, height: 32 } },
/// ];
///
/// let mut clip = AnimationClip::new(
///     "Two Frames".to_string(),
///     &frames,
///     0,
///     Direction::Forward,
///     PlayMode::Loop
/// );
///
/// assert_eq!(clip.get_cell(), Some(0));
/// clip.update(800.);
///
/// assert_eq!(clip.get_cell(), Some(0));
/// clip.update(800.);
///
/// // as playback progresses, we get different frames as a return
/// assert_eq!(clip.get_cell(), Some(1));
/// clip.update(800.);
///
/// // and as the "play head" extends beyond the total duration of the clip, it'll loop back
/// // around to the start. This wrapping behaviour can be customized via the `Direction` parameter.
/// assert_eq!(clip.get_cell(), Some(0));
/// ```
#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub name: String,
    pub current_time: Delta, // represents the "play head"
    pub direction: Direction,
    pub duration: Delta,
    cells: Vec<CellInfo>,
    mode: PlayMode,
    pub drained: bool,
}


impl AnimationClip {
    pub fn new(template: &AnimationClipTemplate, play_mode: PlayMode) -> Self {

        AnimationClip {
            name: template.name.to_owned(),
            current_time: 0.,
            direction: template.direction.clone(),
            duration: template.duration,
            cells: template.cells.clone(),
            mode: play_mode,
            drained: false,
        }
    }

    pub fn update(&mut self, dt: Delta) {
        let updated = self.current_time + dt;

        self.current_time = if updated > self.duration {
            self.drained = match self.mode {
                PlayMode::OneShot | PlayMode::Hold => true,
                _ => false,
            };

            updated % self.duration
        } else {
            updated
        };
    }

    /// Explicitly sets the current time of the clip and adjusts the internal
    /// `AnimationClip.drained` value based on the clip's mode and whether the new time is larger
    /// than the duration.
    pub fn set_time(&mut self, time: Delta) {
        self.current_time = if time > self.duration {
            self.drained = self.mode != PlayMode::Loop;
            time % self.duration
        } else {
            time
        }

    }

    /// Put the play head back to the start of the clip.
    pub fn reset(&mut self) {
        self.set_time(0.);
    }

    /// Returns the cell index for the current time of the clip or None if the clip is over.
    pub fn get_cell(&self) -> Option<usize> {

        if self.drained {
            return if self.mode == PlayMode::OneShot {
                None
            } else {
                Some(self.cells.last().unwrap().idx)
            };
        }

        let mut remaining_time = self.current_time;

        if self.mode == PlayMode::Loop {
            // FIXME: dupe code caused by iter() and cycle() having different types (otherwise
            // would return a generic iter from match and loop over after).
            for cell in self.cells.iter().cycle() {
                remaining_time -= cell.duration as Delta;
                if remaining_time <= 0. {
                    return Some(cell.idx);
                }
            }
        } else {
            for cell in self.cells.iter() {
                remaining_time -= cell.duration as Delta;
                if remaining_time <= 0. {
                    return Some(cell.idx);
                }
            }
        }

        if self.mode == PlayMode::Hold {
            Some(self.cells.len() - 1)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ClipStore {
    store: HashMap<String, AnimationClipTemplate>,
}


pub type SpriteSheetData = aseprite::ExportData;

impl ClipStore {
    pub fn new(data: &SpriteSheetData) -> Self {
        ClipStore {
            store: {
                let mut clips = HashMap::new();

                for tag in &data.meta.frame_tags {

                    let direction = match tag.direction.as_ref() {
                        "forward" => Direction::Forward,
                        "reverse" => Direction::Reverse,
                        "pingpong" => Direction::PingPong,
                        _ => Direction::Unknown,
                    };
                    let frames: &[Frame] = &data.frames[tag.from..tag.to + 1];
                    clips.insert(
                        tag.name.clone(),
                        AnimationClipTemplate::new(tag.name.clone(), frames, direction, tag.from),
                    );
                }

                clips
            },
        }
    }

    pub fn create(&self, key: &str, mode: PlayMode) -> Option<AnimationClip> {
        self.store.get(key).map(|x| AnimationClip::new(x, mode))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_from_file() {
        let sheet = SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json");
        let clips = ClipStore::new(&sheet);

        let alpha = clips.create("Alpha", PlayMode::Loop).unwrap();
        let beta = clips.create("Beta", PlayMode::Loop).unwrap();
        let gamma = clips.create("Gamma", PlayMode::Loop).unwrap();
        assert_eq!(alpha.get_cell(), Some(0));
        assert_eq!(beta.get_cell(), Some(10));
        assert_eq!(gamma.get_cell(), Some(20));
    }

    #[test]
    fn test_clips_are_distinct() {
        let sheet = SpriteSheetData::from_file("resources/numbers/numbers-matrix-tags.array.json");
        let clips = ClipStore::new(&sheet);


        // Each time we get a named clip, we're creating a new instance, and each have their
        // own internal clock.
        let mut alpha1 = clips.create("Alpha", PlayMode::Loop).unwrap();
        let mut alpha2 = clips.create("Alpha", PlayMode::Loop).unwrap();

        alpha1.update(20.);
        alpha2.update(120.);

        assert_eq!(alpha1.get_cell(), Some(0));
        assert_eq!(alpha2.get_cell(), Some(1));
    }

    #[test]
    fn test_clip_cell_count() {
        let sheet = get_two_sheet();
        let clips = ClipStore::new(&sheet);

        let alpha1 = clips.create("Alpha", PlayMode::Loop).unwrap();
        assert_eq!(alpha1.cells.len(), 2);
    }

    #[test]
    fn test_clip_duration() {
        let sheet = get_two_sheet();
        let clips = ClipStore::new(&sheet);

        let alpha1 = clips.create("Alpha", PlayMode::Loop).unwrap();
        assert!((alpha1.duration - 30.).abs() < 0.1);
    }

    #[test]
    fn test_oneshot_bounds() {
        let sheet = get_two_sheet();
        let clips = ClipStore::new(&sheet);


        let mut alpha1 = clips.create("Alpha", PlayMode::OneShot).unwrap();

        assert_eq!(alpha1.get_cell(), Some(0));

        alpha1.update(10.);
        assert_eq!(alpha1.get_cell(), Some(0));

        alpha1.update(1.);
        assert_eq!(alpha1.get_cell(), Some(1));

        alpha1.update(19.);
        assert_eq!(alpha1.get_cell(), Some(1));

        // we should be at the end of the clip at this point
        assert!((alpha1.current_time - alpha1.duration).abs() < 0.1);


        alpha1.update(1.);
        assert_eq!(alpha1.get_cell(), None);

    }

    #[test]
    fn test_hold_bounds() {
        let sheet = get_two_sheet();
        let clips = ClipStore::new(&sheet);


        let mut alpha1 = clips.create("Alpha", PlayMode::Hold).unwrap();

        assert_eq!(alpha1.get_cell(), Some(0));

        alpha1.update(10.);
        assert_eq!(alpha1.get_cell(), Some(0));

        alpha1.update(1.);
        assert_eq!(alpha1.get_cell(), Some(1));

        alpha1.update(19.);
        assert_eq!(alpha1.get_cell(), Some(1));

        // we should be at the end of the clip at this point
        assert!((alpha1.current_time - alpha1.duration).abs() < 0.1);
        assert_eq!(alpha1.drained, false);

        alpha1.update(1.);
        assert_eq!(alpha1.drained, true);

        assert_eq!(alpha1.get_cell(), Some(1));
    }

    #[test]
    fn test_deep_clips_report_correct_index() {

        let sheet = get_pitcher_sheet();
        let clips = ClipStore::new(&sheet);


        let mut not_ready = clips.create("Not Ready", PlayMode::OneShot).unwrap();

        not_ready.update(100.);
        assert_eq!(not_ready.get_cell(), Some(18));
        not_ready.update(100.);
        assert_eq!(not_ready.get_cell(), Some(19));
        not_ready.update(100.);
        assert_eq!(not_ready.get_cell(), Some(20));
        not_ready.update(100.);
        assert_eq!(not_ready.get_cell(), None);

        //        let mut pitching = clips.create("Pitching", PlayMode::OneShot);

    }

    /// Generates a new sprite sheet with a 2 frame clip.
    fn get_two_sheet() -> SpriteSheetData {
        aseprite::ExportData::parse_str(
            r#"{
          "frames": [
            {
              "frame": { "x": 0, "y": 0, "w": 32, "h": 32 },
              "duration": 10
            },
            {
              "frame": { "x": 32, "y": 0, "w": 32, "h": 32 },
              "duration": 20
            }
          ],
          "meta": {
            "size": { "w": 64, "h": 32 },
            "frameTags": [
              { "name": "Alpha", "from": 0, "to": 1, "direction": "forward" }
            ]
          }
        }"#,
        )
    }
    /// a real-world usage from LD38
    fn get_pitcher_sheet() -> SpriteSheetData {
        aseprite::ExportData::parse_str(
            r#"{
            "frames": [
                {"frame": { "x": 0, "y": 0, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 0, "y": 257, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 0, "y": 514, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 257, "y": 0, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 257, "y": 257, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 257, "y": 514, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 514, "y": 0, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 514, "y": 257, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 514, "y": 514, "w": 256, "h": 256 }, "duration": 1000},
                {"frame": { "x": 771, "y": 0, "w": 256, "h": 256 }, "duration": 200},
                {"frame": { "x": 771, "y": 257, "w": 256, "h": 256 }, "duration": 400},
                {"frame": { "x": 771, "y": 514, "w": 256, "h": 256 }, "duration": 200},
                {"frame": { "x": 1028, "y": 0, "w": 256, "h": 256 }, "duration": 150},
                {"frame": { "x": 1028, "y": 257, "w": 256, "h": 256 }, "duration": 150},
                {"frame": { "x": 1028, "y": 514, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 1285, "y": 0, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 1285, "y": 257, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 1285, "y": 514, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 1542, "y": 0, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 1542, "y": 257, "w": 256, "h": 256 }, "duration": 100},
                {"frame": { "x": 1542, "y": 514, "w": 256, "h": 256 }, "duration": 100}
            ],
              "meta": {
                "size": { "w": 2048, "h": 1024 },
                "frameTags": [
                  { "name": "Ready", "from": 0, "to": 7, "direction": "forward" },
                  { "name": "Winding", "from": 8, "to": 13, "direction": "forward" },
                  { "name": "Pitching", "from": 14, "to": 17, "direction": "forward" },
                  { "name": "Not Ready", "from": 18, "to": 20, "direction": "forward" }
                ]
              }
            }"#,
        )
    }
}
