use ggez::{graphics, Context, GameResult};

use crate::{ball, bar};

pub struct Game {
    bar: bar::Bar,
    ball: ball::Ball,
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, graphics::BLACK);
        self.bar.draw(_ctx)?;
        self.ball.draw(_ctx)?;
        graphics::present(_ctx)?;
        Ok(())
    }
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let bar = bar::Bar::new(_ctx);
        let ball = ball::Ball::new(_ctx);
        Game { bar, ball }
    }
}
