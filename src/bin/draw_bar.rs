use ggez::{Context, ContextBuilder, event, GameResult, graphics};
use ggez::nalgebra as na;

struct State {
    bar_texture: graphics::Image,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, graphics::BLACK);
        graphics::draw(_ctx, &self.bar_texture,
                       graphics::DrawParam::default()
                           .dest(na::Point2::new(0.0, 0.0))
                           .scale(na::Vector2::new(1.0, 1.0)),
        )?;
        graphics::present(_ctx)?;
        Ok(())
    }
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (ref mut _ctx, ref mut event_loop) = ContextBuilder::new("bricks", "dean")
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let bar_texture = graphics::Image::new(_ctx, "/bar.png").unwrap();
    let state = &mut State { bar_texture };

    event::run(_ctx, event_loop, state).unwrap();
}