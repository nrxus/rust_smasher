use animation::Animation;

use glm;
use moho::resource_manager::{Renderer, ResourceManager};

use std::rc::Rc;
use std::time::Duration;

pub struct DrawableMeteor<R: Renderer> {
    meteor: Rc<R::Texture>,
    explosion: Rc<R::Texture>,
    animation: Animation,
    meteor_dims: glm::UVec2,
    explosion_dims: glm::UVec2,
    max_coords: glm::UVec2,
}

impl<R: Renderer> DrawableMeteor<R> {
    pub fn new(max_coords: glm::UVec2,
               resource_manager: &mut ResourceManager<R>)
               -> Result<Self, String> {
        let meteor = resource_manager.load_texture("resources/meteor.png")?;
        let explosion = resource_manager.load_texture("resources/explosion_large.png")?;
        let explosion_dims = glm::uvec2(explosion.dims.x / 8, explosion.dims.y);
        let frame_duration = Duration::from_millis(80_u64);
        let animation = Animation::new(8, frame_duration, explosion.dims, false);

        let drawable = DrawableMeteor {
            meteor: meteor.texture,
            explosion: explosion.texture,
            animation: animation,
            meteor_dims: meteor.dims,
            explosion_dims: explosion_dims,
            max_coords: max_coords,
        };

        Ok(drawable)
    }

    pub fn animate_explosion(&mut self) -> bool {
        self.animation.update();
        self.animation.is_active()
    }

    pub fn meteor_dims(&self) -> glm::UVec2 {
        self.meteor_dims
    }

    pub fn draw_meteor(&self,
                       center: glm::IVec2,
                       renderer: &mut ResourceManager<R>)
                       -> Result<(), String> {
        let max_coords = Some(self.max_coords);
        renderer.draw_from_center(&*self.meteor, None, center, self.meteor_dims, max_coords)
    }

    pub fn draw_explosion(&self,
                          center: glm::IVec2,
                          renderer: &mut ResourceManager<R>)
                          -> Result<(), String> {
        let max_coords = Some(self.max_coords);
        let src_rect = Some(self.animation.src_rect());
        let dims = self.explosion_dims;
        renderer.draw_from_center(&*self.explosion, src_rect, center, dims, max_coords)
    }
}
