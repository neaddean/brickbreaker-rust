use ggez::nalgebra as na;
use ggez::{graphics, Context};
use specs::{join::Join, ReadStorage, System};

use crate::components::*;

pub struct RenderingSystem<'a> {
    pub ctx: &'a mut Context,
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
                graphics::DrawParam::new().dest(na::Point2::new(position.x, position.y));
            graphics::draw(self.ctx, &renderable.texture, draw_params).unwrap();
        }

        graphics::present(self.ctx).unwrap();
    }
}
