use ggez::{Context, GameResult, graphics};
use ggez::nalgebra as na;

pub struct Bar {
    texture: graphics::Image,
    position: na::Point2<f32>,
    scale: na::Vector2<f32>,
}

impl Bar {
    pub fn new(_ctx: &mut Context) -> Self {
        let texture = graphics::Image::new(_ctx, "/bar.png").unwrap();
        Bar {
            texture,
            position: na::Point2::new(0.0, 0.0),
            scale: na::Vector2::new(1.0, 1.0),
        }
    }

    pub fn draw(&self, _ctx: &mut Context) -> GameResult<()> {
        graphics::draw(_ctx, &self.texture,
                       graphics::DrawParam::default()
                           .dest(self.position)
                           .scale(self.scale),
        )?;
        Ok(())
    }
}