extern crate sdl2;

use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;
use sdl2_image::LoadTexture;
use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::rect;

trait Renderer {
    type Texture;
    fn load_texture(&self, path: &Path) -> Result<Self::Texture, String>;
    fn draw(&mut self,
            image: Rc<Self::Texture>,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<(), String>;
}

impl<'a> Renderer for SdlRenderer<'a> {
    type Texture = SdlTexture;

    fn load_texture(&self, path: &Path) -> Result<SdlTexture, String> {
        LoadTexture::load_texture(self, path)
    }

    fn draw(&mut self,
            image: Rc<SdlTexture>,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<(), String> {
        self.copy(&*image, src, dst)
    }
}

struct ResourceManager<'a, I: Renderer> {
    texture_cache: RefCell<HashMap<&'a str, Rc<I::Texture>>>,
    image_loader: I,
}

impl<'a, I: Renderer> ResourceManager<'a, I> {
    pub fn new(image_loader: I) -> Self {
        ResourceManager {
            texture_cache: RefCell::new(HashMap::new()),
            image_loader: image_loader,
        }
    }

    pub fn load_texture(&self, path: &'a str) -> Result<Rc<I::Texture>, String> {
        {
            let cache = self.texture_cache.borrow();
            let image = cache.get(path);
            if let Some(x) = image {
                return Ok(x.clone());
            }
        }
        let mut cache = self.texture_cache.borrow_mut();
        let image_path = Path::new(path);
        let image = Rc::new(try!(self.image_loader.load_texture(image_path)));
        cache.insert(path, image.clone());
        Ok(image.clone())
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use std::cell::RefCell;
    use std::rc::Rc;
    use sdl2::rect;
    use super::Renderer;
    use super::ResourceManager;

    #[derive(Debug)]
    struct MockTexture {
        path: String,
    }

    struct RendererTracker {
        load_count: u16,
        last_img: Rc<MockTexture>,
        last_src: Option<rect::Rect>,
        last_dst: Option<rect::Rect>,
    }

    impl RendererTracker {
        fn new() -> Self {
            RendererTracker {
                load_count: 0,
                last_img: Rc::new(MockTexture { path: "NULL".into() }),
                last_dst: None,
                last_src: None,
            }
        }
    }

    struct MockRenderer {
        error: Option<String>,
        tracker: Rc<RefCell<RendererTracker>>,
    }

    impl Renderer for MockRenderer {
        type Texture = MockTexture;

        fn load_texture(&self, path: &Path) -> Result<MockTexture, String> {
            self.tracker.borrow_mut().load_count += 1;
            match self.error {
                None => Ok(MockTexture { path: path.to_str().unwrap_or("").into() }),
                Some(ref e) => Err(e.clone()),
            }
        }

        fn draw(&mut self,
                image: Rc<MockTexture>,
                src: Option<rect::Rect>,
                dst: Option<rect::Rect>)
                -> Result<(), String> {
            match self.error {
                None => {
                    let mut tracker = self.tracker.borrow_mut();
                    tracker.last_img = image;
                    tracker.last_src = src;
                    tracker.last_dst = dst;
                    Ok(())
                }
                Some(ref e) => Err(e.clone()),
            }
        }
    }

    fn new_subject<'a>(error: Option<String>)
                       -> (ResourceManager<'a, MockRenderer>, Rc<RefCell<RendererTracker>>) {
        let tracker = Rc::new(RefCell::new(RendererTracker::new()));
        let image_loader = MockRenderer {
            error: error,
            tracker: tracker.clone(),
        };

        let subject = ResourceManager::new(image_loader);
        (subject, tracker)
    }

    #[test]
    fn loads_image() {
        let (subject, tracker) = new_subject(None);
        let image = subject.load_texture("mypath/").unwrap();
        assert_eq!(image.path, String::from("mypath/"));
        assert_eq!(tracker.borrow().load_count, 1);
    }

    #[test]
    fn returns_error() {
        let (subject, tracker) = new_subject(Some("FAIL".into()));
        let image = subject.load_texture("mypath/");
        assert_eq!(image.err(), Some("FAIL".into()));
        assert_eq!(tracker.borrow().load_count, 1);
    }

    #[test]
    fn caches_images() {
        let (subject, tracker) = new_subject(None);

        // get a new image - number of calls is 1
        let image1 = subject.load_texture("mypath/1").unwrap();
        assert_eq!(image1.path, String::from("mypath/1"));
        assert_eq!(tracker.borrow().load_count, 1);

        // load the same image - number of calls should still be 1
        let image2 = subject.load_texture("mypath/1").unwrap();
        assert_eq!(image2.path, String::from("mypath/1"));
        assert_eq!(tracker.borrow().load_count, 1);

        // load a different image - number of calls should increase
        let image3 = subject.load_texture("mypath/2").unwrap();
        assert_eq!(image3.path, String::from("mypath/2"));
        assert_eq!(tracker.borrow().load_count, 2);
    }

    #[test]
    fn draws_images() {
        let (subject, tracker) = new_subject(None);
        let image = subject.load_texture("mypath/").unwrap();
    }
}
