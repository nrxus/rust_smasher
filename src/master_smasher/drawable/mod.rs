pub mod animation;
pub mod asset;
pub mod animation_data;

pub use self::animation_data::AnimationData;
pub use self::animation::Animation;
pub use self::asset::Asset;

use errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

pub trait Drawable<R: GameRenderer> {
    fn draw(&self, renderer: &mut R) -> Result<()>;
}

pub trait GameRenderer: Sized {
    fn render<D: Drawable<Self>>(&mut self, drawable: &D) -> Result<()>;
    fn render_all<D: Drawable<Self>>(&mut self, drawable: &[D]) -> Result<()>;
}

impl<R: Renderer> GameRenderer for ResourceManager<R> {
    fn render<D: Drawable<ResourceManager<R>>>(&mut self, drawable: &D) -> Result<()> {
        drawable.draw(self)
    }

    fn render_all<D: Drawable<ResourceManager<R>>>(&mut self, drawable: &[D]) -> Result<()> {
        drawable.iter().map(|d| d.draw(self)).take_while(Result::is_ok).last().unwrap_or(Ok(()))
    }
}
