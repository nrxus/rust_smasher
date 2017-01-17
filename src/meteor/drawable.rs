use animation::Animation;

use glm;
use glm::ext::normalize_to;
use moho::resource_manager::{Renderer, ResourceManager};
use sdl2::rect;

use std::rc::Rc;
use std::time::Duration;

pub struct Drawable<R: Renderer> {
    pub center: glm::IVec2,
    meteor_dims: glm::UVec2,
    explosion_dims: glm::UVec2,
    max_coords: glm::UVec2,
    animation: Animation,
    meteor: Rc<R::Texture>,
    explosion: Rc<R::Texture>,
    rects: [rect::Rect; 10],
}

impl<R: Renderer> Drawable<R> {
    pub fn new(center: glm::IVec2,
               max_coords: glm::UVec2,
               resource_manager: &mut ResourceManager<R>)
               -> Result<Self, String> {
        const NUM_FRAMES: u32 = 8;
        let meteor = resource_manager.load_texture("resources/meteor.png")?;
        let explosion = resource_manager.load_texture("resources/explosion_large.png")?;
        let explosion_dims = glm::uvec2(explosion.dims.x / NUM_FRAMES, explosion.dims.y);
        let frame_duration = Duration::from_millis(80_u64);
        let animation = Animation::new(NUM_FRAMES, frame_duration, explosion.dims, false);

        let drawable = Drawable {
            center: center,
            meteor_dims: meteor.dims,
            explosion_dims: explosion_dims,
            max_coords: max_coords,
            animation: animation,
            meteor: meteor.texture,
            explosion: explosion.texture,
            rects: [rect::Rect::new(0, 0, 5, 5); 10],
        };

        Ok(drawable)
    }

    pub fn animate_explosion(&mut self) -> bool {
        self.animation.update();
        self.animation.is_active()
    }

    pub fn update_launch_vector(&mut self, target: glm::IVec2) {
        let target = glm::to_dvec2(target);
        let center = glm::to_dvec2(self.center);
        let distance = target - center;
        let offset = self.meteor_dims.x / 2 + 10;
        let offset_vector = normalize_to(distance, offset as f64);
        let anchor_point = center + offset_vector;
        let step = (target - anchor_point) / (self.rects.len() as f64);

        for (i, rect) in self.rects.iter_mut().enumerate() {
            let point = glm::to_ivec2(anchor_point + (step * i as f64));
            rect.center_on((point.x, point.y));
        }
    }

    pub fn meteor_dims(&self) -> glm::UVec2 {
        self.meteor_dims
    }

    pub fn draw_unlaunched(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        self.draw_meteor(renderer)?;
        renderer.fill_rects(&self.rects)
    }

    pub fn draw_meteor(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let max_coords = Some(self.max_coords);
        let texture = &*self.meteor;
        renderer.draw_from_center(texture, None, self.center, self.meteor_dims, max_coords)
    }

    pub fn draw_explosion(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let max_coords = Some(self.max_coords);
        let texture = &*self.explosion;
        let src_rect = Some(self.animation.src_rect());
        let dims = self.explosion_dims;
        renderer.draw_from_center(texture, src_rect, self.center, dims, max_coords)
    }
}
