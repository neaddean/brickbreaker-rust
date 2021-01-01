use ggez::{Context, graphics};
use ggez::nalgebra as na;
use specs::{join::Join, ReadStorage, System};

use crate::components::*;

pub struct RenderingSystem<'a> {
    pub ctx: &'a mut Context,
}

impl RenderingSystem<'_> {
    fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = na::Point2::new(x, y);
        let color = Some(graphics::Color::new(0.0, 1.0, 0.0, 1.0));
        let dimensions = na::Point2::new(2.0, 2.0);

        graphics::queue_text(self.ctx, &text, dimensions, color);
        graphics::draw_queued_text(
            self.ctx,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
            .expect("expected drawing queued text");
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Renderable>);


    fn run(&mut self, data: Self::SystemData) {
        graphics::clear(self.ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let (positions, renderables) = data;
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let draw_params =
                graphics::DrawParam::new()
                    .dest(na::Point2::new(position.x, position.y))
                    .offset(na::Point2::new(0.5, 0.5));
            let texture = graphics::Image::new(self.ctx, &renderable.texture).unwrap();
            graphics::draw(self.ctx, &texture, draw_params).unwrap();
        }

        self.draw_text(format!("{:0.2}", ggez::timer::fps(self.ctx)).as_str(), 0.0, 0.0);

        graphics::present(self.ctx).unwrap();
    }
}
