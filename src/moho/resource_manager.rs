extern crate sdl2;

use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;
use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

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
    texture_cache: RefCell<HashMap<&'a str, Rc<I::Image>>>,
    image_loader: I,
}

impl<'a, I: ImageLoader> ResourceManager<'a, I> {
    pub fn new(image_loader: I) -> Self {
        ResourceManager {
            texture_cache: RefCell::new(HashMap::new()),
            image_loader: image_loader,
        }
    }
    pub fn getTexture(&self, path: &'a str) -> Result<Rc<I::Image>, String> {
        {
            let cache = self.texture_cache.borrow();
            let image = cache.get(path);
            if let Some(x) = image {
                return Ok(x.clone());
            }
        }
        let mut cache = self.texture_cache.borrow_mut();
        let image_path = Path::new(path);
        let image = Rc::new(try!(self.image_loader.load_from_path(image_path)));
        cache.insert(path, image.clone());
        Ok(image.clone())
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use std::cell::RefCell;
    use super::ImageLoader;
    use super::ResourceManager;

    #[derive(Debug)]
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
        let image_loader = MockImageLoader {
            load_count: RefCell::new(0),
            error: None,
        };
        let subject = ResourceManager::new(image_loader);
        let image = subject.getTexture("mypath/").unwrap();
        assert_eq!(image.path, String::from("mypath/"));
        assert_eq!(image.id, 1);
    }

    #[test]
    fn returns_error() {
        let image_loader = MockImageLoader {
            load_count: RefCell::new(0),
            error: Some("FAIL".into()),
        };
        let subject = ResourceManager::new(image_loader);
        let image = subject.getTexture("mypath/");
        assert_eq!(image.err(), Some("FAIL".into()));
    }

    #[test]
    fn caches_images() {
        let image_loader = MockImageLoader {
            load_count: RefCell::new(0),
            error: None,
        };
        let subject = ResourceManager::new(image_loader);

        // get a new image - ID will be 1
        let image1 = subject.getTexture("mypath/1").unwrap();
        assert_eq!(image1.path, String::from("mypath/1"));
        assert_eq!(image1.id, 1);

        // get the image again - ID should not change
        let image2 = subject.getTexture("mypath/1").unwrap();
        assert_eq!(image2.path, String::from("mypath/1"));
        assert_eq!(image2.id, 1);

        // get another time - ID should be a new one
        let image3 = subject.getTexture("mypath/2").unwrap();
        assert_eq!(image3.path, String::from("mypath/2"));
        assert_eq!(image3.id, 2);
    }
}
