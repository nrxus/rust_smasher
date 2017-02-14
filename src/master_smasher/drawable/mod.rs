pub mod animation;
pub mod asset;
pub mod animation_data;

pub use self::animation_data::AnimationData;
pub use self::animation::Animation;
pub use self::asset::Asset;

use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use sdl2::rect;

pub enum Drawable<'a> {
    Asset(&'a Asset),
    Rectangles(&'a [rect::Rect]),
}

impl<'a> Drawable<'a> {
    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match *self {
            Drawable::Asset(a) => renderer.draw(a.texture_id, Some(a.dst_rect), a.src_rect),
            Drawable::Rectangles(r) => renderer.fill_rects(r),
        }
    }
}
