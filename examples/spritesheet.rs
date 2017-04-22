
extern crate omn_labs;

use omn_labs::sprites::{SpriteSheetData, AnimationClip, PlayMode};

extern crate time;
extern crate specs;
extern crate ggez;
extern crate rand;

use omn_labs::assets;

use std::time::Duration;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::DrawParam;
use assets::AssetBundle;


struct MainState {
    clip: AnimationClip,
    sheet: SpriteSheetData,
    assets: AssetBundle,
}


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        ctx.print_resource_stats();
        let assets = AssetBundle::new(ctx, &vec!["numbers/numbers-matrix.png"]);

        let sheet = SpriteSheetData::from_file("examples/resources/numbers/numbers-matrix-tags.array.json");
        let s = MainState {
            clip: sheet.clips.create("Alpha", PlayMode::Loop).unwrap(),
            sheet: sheet,
            assets: assets,
        };
        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        let delta_millis = _dt.subsec_nanos() as f32 / 1e6;
        self.clip.update(delta_millis);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let atlas = self.assets.get_image(ctx, "numbers/numbers-matrix.png");
        let w = atlas.width() as f32;
        let h = atlas.height() as f32;
        let cell = &self.sheet.cells[self.clip.get_cell().unwrap()];
        let param = DrawParam {
            src: graphics::Rect::new(
                cell.bbox.x as f32 / w,
                cell.bbox.y as f32 / h,
                cell.bbox.width as f32 / w,
                cell.bbox.height as f32 / h),
            dest: graphics::Point::new(160., 120.),
            scale: graphics::Point::new(1.5, 1.5),
            ..Default::default()
        };

        graphics::draw_ex(ctx, atlas,  param)?;
        graphics::present(ctx);
        Ok(())
    }
}


pub fn main() {

    let mut conf = conf::Conf::new();
    conf.window_width = 320;
    conf.window_height = 240;
    conf.window_title = "Omn Labs RS".to_string();

    println!("Starting with default config: {:#?}", conf);

    let ctx = &mut Context::load_from_conf("Omn Labs", "omnlabs", conf).unwrap();

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
