extern crate sdl2;

use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;
use std::path::Path;
use std::collections::HashMap;

trait ImageLoader {
    type Image;
    fn load_from_path(&self, path: &Path) -> Result<Self::Image, String>;
}

impl<'a> ImageLoader for Renderer<'a> {
    type Image = Texture;
    fn load_from_path(&self, path: &Path) -> Result<Texture, String> {
        self.load_texture(path)
    }
}

struct ResourceManager<'a, I: ImageLoader> {
    texture_cache: HashMap<&'a str, I::Image>,
    image_loader: I,
}

impl<'a, I: ImageLoader> ResourceManager<'a, I> {
    pub fn new(image_loader: I) -> Self {
        ResourceManager {
            texture_cache: HashMap::new(),
            image_loader: image_loader,
        }
    }
    pub fn getTexture(&mut self, path: &'a str) -> Result<&I::Image, String> {
        if !self.texture_cache.contains_key(path) {
            let image_path = Path::new(path);
            let texture = try!(self.image_loader.load_from_path(image_path));
            self.texture_cache.insert(path, texture);
        }
        self.texture_cache.get(path).ok_or("Texture Cache Error: No such path".into())
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use sdl2::render::Texture;
    use std::cell::{RefCell, Ref};
    use super::ImageLoader;
    use super::ResourceManager;

    struct MockImage {
        pub path: String,
        pub id: u16,
    }

    struct MockImageLoader {
        pub load_count: RefCell<u16>,
        pub error: Option<String>,
    }

    impl ImageLoader for MockImageLoader {
        type Image = MockImage;

        fn load_from_path(&self, path: &Path) -> Result<MockImage, String> {
            *self.load_count.borrow_mut() += 1;
            match self.error {
                None => {
                    Ok(MockImage {
                        path: path.to_str().unwrap_or("").into(),
                        id: *self.load_count.borrow(),
                    })
                }
                Some(ref e) => Err(e.clone()),
            }
        }
    }

    #[test]
    fn loads_image() {
        let imageLoader = MockImageLoader {
            load_count: RefCell::new(0),
            error: None,
        };
        let mut subject = ResourceManager::new(imageLoader);
        let image = subject.getTexture("mypath/").unwrap();
        assert_eq!(image.path, String::from("mypath/"));
        assert_eq!(image.id, 1);
    }

    #[test]
    fn returns_error() {
        let imageLoader = MockImageLoader {
            load_count: RefCell::new(0),
            error: Some("FAIL".into()),
        };
        let mut subject = ResourceManager::new(imageLoader);
        let image = subject.getTexture("mypath/");
        assert_eq!(image.err(), Some("FAIL".into()));
    }

    #[test]
    fn caches_images() {
        let imageLoader = MockImageLoader {
            load_count: RefCell::new(0),
            error: None,
        };

        let mut subject = ResourceManager::new(imageLoader);

        {
            let image = subject.getTexture("mypath/1").unwrap();
            assert_eq!(image.path, String::from("mypath/1"));
            assert_eq!(image.id, 1);
        }


        // no error because image is already cached
        {
            let image = subject.getTexture("mypath/1").unwrap();
            assert_eq!(image.path, String::from("mypath/1"));
            assert_eq!(image.id, 1);
        }

        // no error because image is already cached
        let image = subject.getTexture("mypath/2").unwrap();
        assert_eq!(image.path, String::from("mypath/2"));
        assert_eq!(image.id, 2);
    }
}
