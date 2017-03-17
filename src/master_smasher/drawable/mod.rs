pub mod animation;
pub mod asset;
pub mod animation_data;

pub use self::animation_data::AnimationData;
pub use self::animation::Animation;
pub use self::asset::Asset;

use errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};
use moho::tile_sheet::Tile;

use std::slice::Iter;

use glm;

pub trait TryIterator {
    type Item;
    fn try<F>(self, action: F) -> Result<()> where F: FnMut(&Self::Item) -> Result<()>;
}

impl<'a, T> TryIterator for Iter<'a, T> {
    type Item = T;
    fn try<F>(self, action: F) -> Result<()>
        where F: FnMut(&T) -> Result<()>
    {
        self.map(action)
            .take_while(Result::is_ok)
            .last()
            .unwrap_or(Ok(()))
    }
}

pub trait Drawable<R: GameRenderer> {
    fn draw(&self, dst_rect: glm::IVec4, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R: GameRenderer> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait GameRenderer: Sized {
    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }

    fn render<D: Drawable<Self>>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()> {
        drawable.draw(dst_rect, self)
    }
}

impl<R: Renderer> GameRenderer for ResourceManager<R> {}

impl<R: Renderer> Scene<ResourceManager<R>> for Texture {
    fn show(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw(self.id, None, None).map_err(Into::into)
    }
}

impl<R: Renderer> Drawable<ResourceManager<R>> for Texture {
    fn draw(&self, dst_rect: glm::IVec4, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw(self.id, Some(dst_rect), None).map_err(Into::into)
    }
}

impl<R: Renderer> Scene<ResourceManager<R>> for Tile {
    fn show(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw(self.id, None, Some(self.src)).map_err(Into::into)
    }
}

impl<R: Renderer> Drawable<ResourceManager<R>> for Tile {
    fn draw(&self, dst_rect: glm::IVec4, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw(self.id, Some(dst_rect), Some(self.src)).map_err(Into::into)
    }
}
