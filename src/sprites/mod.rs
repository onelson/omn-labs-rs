//! The `sprites` module contains types and functions for managing "playback" of frame sequences
//! over time.

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FrameTag {
    pub name: String,
    pub from: usize,
    pub to: usize,
    pub direction: String
}

pub type Delta = f32;

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use omn_labs::sprites::{AnimationClip, Delta, Frame, Region};
///
/// let clip = AnimationClip::new(vec![
///     Frame { duration: 1000, bbox: Region { x: 0, y: 0, width: 32, height: 32 } },
///     Frame { duration: 1000, bbox: Region { x: 32, y: 0, width: 32, height: 32 } },
/// ]);
///
/// let mut current_time = 0.;
/// assert_eq!(clip.get_frame(current_time), 0);
/// current_time += 800.;
/// assert_eq!(clip.get_frame(current_time), 0);
/// current_time += 800.;
/// // as playback progresses, we get different frames as a return
/// assert_eq!(clip.get_frame(current_time), 1);
/// current_time += 800.;
/// // and as the "play head" extends beyond the total duration of the clip, it'll loop back
/// // around to the start.
/// assert_eq!(clip.get_frame(current_time), 0);
/// ```
pub struct AnimationClip {
    pub duration: Delta,
    // FIXME: should be a vec of durations. Should pair with a separate object with the frame data?
    // The same frames will likely be part of other clips. Could simply index into an object
    // representing the full sprite sheet.
    pub frames: Vec<Frame>
}


impl AnimationClip {
    pub fn new(frames: Vec<Frame>) -> Self {
        AnimationClip {
            duration: frames.iter().map(|x| x.duration as Delta).sum(),
            frames: frames
        }
    }

    #[allow(dead_code)]
    pub fn get_frame(&self, time: Delta) -> usize {
        let mut remaining_time = {
            if time > self.duration {
                time - self.duration
            } else {
                time
            }
        };

        for frame in self.frames.iter().cycle() {
            remaining_time -= frame.duration as Delta;
            if remaining_time <= 0. { return self.frames.iter().position(|ref x| x == &frame).unwrap(); }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod test {

}
