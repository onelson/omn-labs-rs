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
    duration: i32,
    #[serde(rename="frame")]
    bbox: Region,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FrameTag {
    pub name: String,
    pub from: usize,
    pub to: usize,
    pub direction: String
}

pub type Delta = f32;

pub struct Clip<'a> {
    current_time: Delta,
    duration: Delta,
    frames: Vec<&'a Frame>
}

impl<'a> Clip<'a> {
    fn update(&mut self, dt: Delta) -> () {
        self.current_time += dt;
    }

    #[allow(dead_code)]
    fn get_frame(self) -> &'a Frame {
        let mut remaining_time = (self.current_time as Delta) - self.duration;
        for frame in self.frames.iter().cycle() {
            remaining_time -= frame.duration as Delta;
            if remaining_time <= 0. { return frame; }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod test {

}
