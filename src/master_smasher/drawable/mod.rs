pub mod animation;
pub mod asset;
pub mod animation_data;

pub use self::animation_data::AnimationData;
pub use self::animation::Animation;
pub use self::asset::Asset;

use errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

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
    fn draw(&self, position: glm::IVec2, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R: GameRenderer> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait GameRenderer: Sized {
    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }

    fn draw<D: Drawable<Self>>(&mut self, drawable: &D, position: glm::IVec2) -> Result<()> {
        drawable.draw(position, self)
    }
}

impl<R: Renderer> GameRenderer for ResourceManager<R> {}
