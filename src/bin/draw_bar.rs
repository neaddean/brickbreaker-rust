use ggez::{Context, ContextBuilder, event, GameResult, graphics};


use bricktest::bar;

struct State {
    bar: bar::Bar,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, graphics::BLACK);
        self.bar.draw(_ctx)?;
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

    let bar = bar::Bar::new(_ctx);
    let state = &mut State { bar };

    event::run(_ctx, event_loop, state).unwrap();
}