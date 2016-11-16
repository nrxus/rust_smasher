extern crate moho;
extern crate sdl2;

use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::rect;
use moho::resource_manager::*;

#[test]
fn loads_texture_data() {
    let (subject, tracker) = new_subject(None);
    let texture_data = subject.load_texture("mypath/").unwrap().texture;
    assert_eq!(texture_data.path, String::from("mypath/"));
    assert_eq!(tracker.borrow().load_count, 1);
}

#[test]
fn returns_error() {
    let (subject, tracker) = new_subject(Some("FAIL".into()));
    let texture_data = subject.load_texture("mypath/");
    assert_eq!(texture_data.err(), Some("FAIL".into()));
    assert_eq!(tracker.borrow().load_count, 1);
}

#[test]
fn caches_texture_datas() {
    let (subject, tracker) = new_subject(None);

    // get a new texture_data - number of calls is 1
    let texture_data1 = subject.load_texture("mypath/1").unwrap().texture;
    assert_eq!(texture_data1.path, String::from("mypath/1"));
    assert_eq!(tracker.borrow().load_count, 1);

    // load the same texture_data - number of calls should still be 1
    let texture_data2 = subject.load_texture("mypath/1").unwrap().texture;
    assert_eq!(texture_data2.path, String::from("mypath/1"));
    assert_eq!(tracker.borrow().load_count, 1);

    // load a different texture_data - number of calls should increase
    let texture_data3 = subject.load_texture("mypath/2").unwrap().texture;
    assert_eq!(texture_data3.path, String::from("mypath/2"));
    assert_eq!(tracker.borrow().load_count, 2);
}

#[test]
fn draws_texture_datas() {
    let (subject, tracker) = new_subject(None);
    let texture_data = subject.load_texture("mypath/").unwrap();
}

#[derive(Debug)]
struct MockTexture {
    path: String,
}

struct RendererTracker {
    load_count: u16,
    last_texture: Rc<MockTexture>,
    last_src: Option<rect::Rect>,
    last_dst: Option<rect::Rect>,
}

impl RendererTracker {
    fn new() -> Self {
        RendererTracker {
            load_count: 0,
            last_texture: Rc::new(MockTexture { path: "NULL".into() }),
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

    fn load_texture(&self, path: &Path) -> Result<TextureData<MockTexture>, String> {
        self.tracker.borrow_mut().load_count += 1;
        match self.error {
            None => {
                let texture = MockTexture { path: path.to_str().unwrap_or("").into() };
                Ok(TextureData {
                    texture: Rc::new(texture),
                    width: 48,
                    height: 60,
                })
            }
            Some(ref e) => Err(e.clone()),
        }
    }

    fn copy(&mut self,
            texture: Rc<MockTexture>,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<(), String> {
        match self.error {
            None => {
                let mut tracker = self.tracker.borrow_mut();
                tracker.last_texture = texture;
                tracker.last_src = src;
                tracker.last_dst = dst;
                Ok(())
            }
            Some(ref e) => Err(e.clone()),
        }
    }

    fn clear(&mut self) {}

    fn present(&mut self) {}

    fn output_size(&self) -> Result<(u32, u32), String> {
        Ok((0, 0))
    }
}

fn new_subject<'a>(error: Option<String>)
                   -> (ResourceManager<'a, MockRenderer>, Rc<RefCell<RendererTracker>>) {
    let tracker = Rc::new(RefCell::new(RendererTracker::new()));
    let renderer = MockRenderer {
        error: error,
        tracker: tracker.clone(),
    };

    let subject = ResourceManager::new(renderer);
    (subject, tracker)
}
