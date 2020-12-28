use ggez::{Context, GameResult, graphics};

use crate::bar;

pub struct Game {
    bar: bar::Bar,
}

impl ggez::event::EventHandler for Game {
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

impl Game {
    pub fn new(_ctx: &mut Context) -> Game  {
        let bar = bar::Bar::new(_ctx);
        Game { bar }
    }
}