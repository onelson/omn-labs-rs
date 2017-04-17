//! The `sprites` module contains types and functions for managing playback of frame sequences
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

pub struct FrameRef {
    pub idx: usize,
    pub duration: FrameDuration
}


/// This function adds two to its argument.
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
/// assert_eq!(clip.get_frame(), 0);
/// clip.update(800.);
///
/// assert_eq!(clip.get_frame(), 0);
/// clip.update(800.);
///
/// // as playback progresses, we get different frames as a return
/// assert_eq!(clip.get_frame(), 1);
/// clip.update(800.);
///
/// // and as the "play head" extends beyond the total duration of the clip, it'll loop back
/// // around to the start. This wrapping behaviour can be customized via the `Direction` parameter.
/// assert_eq!(clip.get_frame(), 0);
/// ```
pub struct AnimationClip {
    current_time: Delta,  // represents the "play head"
    pub direction: Direction,
    pub duration: Delta,
    // FIXME: should be a vec of durations. Should pair with a separate object with the frame data?
    // The same frames will likely be part of other clips. Could simply index into an object
    // representing the full sprite sheet. If we support "direction" as a playback option, we'll
    // need something to manage the mapping of indices, especially wrt "ping-pong".
    frames: Vec<FrameRef>
}


impl AnimationClip {
    pub fn new<'a>(frames: &'a Vec<Frame>, direction: Direction) -> Self {

        let frame_data: Vec<FrameRef> = match direction {
            Direction::Forward =>
                frames.iter().enumerate()
                    .map(|(idx, ref x)| FrameRef { idx: idx, duration: x.duration})
                    .collect(),
            Direction::Reverse =>
                frames.iter().enumerate().rev()
                    .map(|(idx, ref x)| FrameRef { idx: idx, duration: x.duration})
                    .collect(),
            // Look at what aseprite does about each end (double frame problem)
            Direction::PingPong =>
                frames.iter().enumerate().chain(frames.iter().enumerate().rev())
                    .map(|(idx, ref x)| FrameRef { idx: idx, duration: x.duration})
                    .collect(),

        };

        AnimationClip {
            current_time: 0.,
            direction: direction,
            duration: frame_data.iter().map(|x| x.duration as Delta).sum(),
            frames: frame_data
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
    pub fn get_frame(&self) -> usize {
        let mut remaining_time = self.current_time;
        for frame in self.frames.iter().cycle() {
            remaining_time -= frame.duration as Delta;
            if remaining_time <= 0. { return frame.idx; }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod test {

}
