use std::collections::HashMap;

use ggez::{Context, graphics};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra as na;
use itertools::Itertools;
use specs::{join::Join, Read, ReadStorage, System};

use crate::components::*;
use crate::resources;

pub struct RenderingSystem<'a> {
    pub ctx: &'a mut Context,
}

impl RenderingSystem<'_> {
    fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let color = Some(graphics::Color::new(0.0, 1.0, 0.0, 1.0));
        let dimensions = na::Point2::new(x, y);
        graphics::queue_text(self.ctx, &text, dimensions, color);
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Renderable>,
                       Read<'a, resources::AssetCache>,
                       Read<'a, resources::GameState>);


    fn run(&mut self, data: Self::SystemData) {
        graphics::clear(self.ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let (positions,
            renderables,
            asset_cache,
            game_state,
        ) = data;
        // let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();

        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<graphics::DrawParam>>> = HashMap::new();

        // Iterate each of the renderables, determine which image path should be rendered
        // at which drawparams, and then add that to the rendering_batches.
        for (position, renderable) in (&positions, &renderables).join() {
            rendering_batches
                .entry(position.z)
                .or_default()
                .entry(renderable.asset_name.to_string())
                .or_default()
                .push(graphics::DrawParam::new()
                    .dest(na::Point2::new(position.x, position.y))
                    .offset(na::Point2::new(0.5, 0.5)));
        }

        // Iterate spritebatches ordered by z and actually render each of them
        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                let texture = asset_cache.cache.get(image_path).unwrap().clone();
                let mut sprite_batch = SpriteBatch::new(texture);

                for draw_param in draw_params.iter() {
                    sprite_batch.add(*draw_param);
                }

                graphics::draw(self.ctx, &sprite_batch, graphics::DrawParam::new()).unwrap();
            }
        }

        if game_state.show_fps {
            self.draw_text(format!("{:0.2}", ggez::timer::fps(self.ctx)).as_str(), 0.0, 0.0);
        }
        graphics::draw_queued_text(
            self.ctx,
            graphics::DrawParam::new().dest(na::Point2::new(0.0, 0.0)),
            None,
            graphics::FilterMode::Linear,
        )
            .expect("expected drawing queued text");
        graphics::present(self.ctx).unwrap();
    }
}
