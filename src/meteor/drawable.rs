use animation::Animation;
use asset::Asset;

use glm;
use glm::ext::normalize_to;
use moho::errors::*;
use moho::frame_animator::FrameAnimator;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use moho::tile_sheet::TileSheet;
use sdl2::rect;

use std::time::Duration;

pub struct Drawable {
    pub center: glm::IVec2,
    max_coords: glm::UVec2,
    explosion: Animation,
    meteor: Asset,
    rects: [rect::Rect; 10],
}

impl Drawable {
    pub fn new<R: Renderer>(center: glm::IVec2,
                            max_coords: glm::UVec2,
                            resource_manager: &ResourceManager<R>)
                            -> Result<Self> {
        const NUM_FRAMES: u32 = 8;
        let meteor = resource_manager.load_texture("resources/meteor.png")?;
        let meteor = Asset::from_texture(&meteor);
        let explosion = resource_manager.load_texture("resources/explosion_large.png")?;
        let explosion_dims = glm::uvec2(explosion.dims.x / NUM_FRAMES, explosion.dims.y);
        let explosion = Asset::new(explosion.id, explosion_dims);
        let frame_duration = Duration::from_millis(80_u64);
        let tile_sheet = TileSheet::new(glm::uvec2(NUM_FRAMES, 1));
        let animator = FrameAnimator::new(NUM_FRAMES, frame_duration, false);
        let explosion = Animation::new(explosion, tile_sheet, animator);

        let drawable = Drawable {
            center: center,
            max_coords: max_coords,
            meteor: meteor,
            explosion: explosion,
            rects: [rect::Rect::new(0, 0, 5, 5); 10],
        };

        Ok(drawable)
    }

    pub fn animate_explosion(&mut self) {
        self.explosion.update();
    }

    pub fn update_launch_vector(&mut self, target: glm::IVec2) {
        let target = glm::to_dvec2(target);
        let center = glm::to_dvec2(self.center);
        let distance = target - center;
        let offset = self.meteor.dimensions.x / 2 + 10;
        let offset_vector = normalize_to(distance, offset as f64);
        let anchor_point = center + offset_vector;
        let step = (target - anchor_point) / (self.rects.len() as f64);

        for (i, rect) in self.rects.iter_mut().enumerate() {
            let point = glm::to_ivec2(anchor_point + (step * i as f64));
            rect.center_on((point.x, point.y));
        }
    }

    pub fn meteor_dims(&self) -> glm::UVec2 {
        self.meteor.dimensions
    }

    pub fn draw_unlaunched<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.draw_meteor(renderer)?;
        renderer.fill_rects(&self.rects)
    }

    pub fn draw_meteor<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.draw(&self.meteor, None, renderer)
    }

    pub fn draw_explosion<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        let src_rect = Some(self.explosion.src_rect());
        self.draw(self.explosion.asset(), src_rect, renderer)
    }

    pub fn is_exploding(&self) -> bool {
        self.explosion.is_active()
    }

    fn draw<R>(&self,
               asset: &Asset,
               src: Option<glm::DVec4>,
               renderer: &mut ResourceManager<R>)
               -> Result<()>
        where R: Renderer
    {
        let max_coords = Some(self.max_coords);
        let rect = asset.dst_rect(self.center);
        renderer.draw(asset.texture_id, Some(rect), src, max_coords)
    }
}
