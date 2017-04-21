//! The `aseprite` module contains types and functions extracting information from the json export
//! data feature provided by [Aseprite](https://www.aseprite.org/).

#[allow(unused_imports)]
use super::{Region, FrameTag, Frame};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dimensions {
    #[serde(rename="w")]
    pub width: i32,
    #[serde(rename="h")]
    pub height: i32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Meta {
    #[serde(rename="frameTags")]
    pub frame_tags: Vec<FrameTag>,
    pub size: Dimensions
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExportData {
    pub frames: Vec<Frame>,
    pub meta: Meta
}

#[cfg(test)]
mod test {
    use serde_json;

    use super::*;

    fn get_alpha() -> ExportData {
        ExportData {
            frames: vec![
                Frame { duration: 1000, bbox: Region { x: 0, y: 0, width: 32, height: 32 } },
                Frame { duration: 1000, bbox: Region { x: 32, y: 0, width: 32, height: 32 } },
            ],
            meta: Meta {
                frame_tags: vec![
                    FrameTag { name: "Alpha".to_string(), from: 0, to: 1, direction: "forward".to_string() }
                ],
                size: Dimensions { width: 64, height: 32 }
            }
        }
    }

    #[test]
    fn test_parse() {
        // json data exported from aseprite will have a bunch of additional fields, but the extras
        // will be ignored.
        let aseprite_data = r#"{
          "frames": [
            {
              "frame": { "x": 0, "y": 0, "w": 32, "h": 32 },
              "duration": 1000
            },
            {
              "frame": { "x": 32, "y": 0, "w": 32, "h": 32 },
              "duration": 1000
            }
          ],
          "meta": {
            "size": { "w": 64, "h": 32 },
            "frameTags": [
              { "name": "Alpha", "from": 0, "to": 1, "direction": "forward" }
            ]
          },
          "total": "garbage"
        }"#;

        let expected = get_alpha();
        let result: ExportData = serde_json::from_str(aseprite_data).unwrap();
        assert_eq!(expected.frames, result.frames);
        assert_eq!(expected.meta.frame_tags, result.meta.frame_tags);
        assert_eq!(expected.meta.size, result.meta.size);
    }
}