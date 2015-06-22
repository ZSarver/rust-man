use std::path::Path;

use sdl2::render::Texture;
use sdl2::render::Renderer;
use sdl2::surface::Surface;
use sdl2::rect::Rect;

use assets;
use physics::Phobject;
use physics::zero;

///A Sprite is a texture with an optional Rect and an angle
pub struct Sprite {
  pub tex: Texture,
  pub rect: Option<Rect>,
  pub angle: f64,
  phys: Option<Phobject>,
  }

impl Sprite {
  ///Constructs a new Sprite from a path
  pub fn new_from_file(p: &str, r: &Renderer) -> Sprite {
    //adjoin asset_dir and p to get the full path
    let asset_path = Path::new(assets::ASSET_STR);
    let full_pathbuf = asset_path.join(p);
    let full_path = full_pathbuf.as_path();
    
    //load surface
    let surf = Surface::from_bmp(full_path)
      .ok().expect("Failed to load asset.");
    
    //create a texture from the surface
    let texture = r.create_texture_from_surface(&surf)
      .ok().expect("Failed to create texture");
        
    Sprite{ tex: texture, rect: Some(Rect::new(100,100,32,64)), angle: 0.0, phys: None }
  }
  
  pub fn mov_x(&mut self, x: i32) {
    match self.rect.as_mut() {
      Some(r) => {
        r.x +=x;
      },
      None => {},
    }
  }
  
  pub fn mov_y(&mut self, y: i32) {
    match self.rect.as_mut() {
      Some(r) => {
        r.y += y;
      },
      None => {},
    }
  }
  
  pub fn mov(&mut self, x: i32, y: i32) {
    self.mov_x(x);
    self.mov_y(y);
  }
  
  pub fn update(&mut self, dt: f64)
  {
    match self.phys.as_mut() {
      Some(p) => {
        p.update(dt);
        match self.rect.as_mut() {
          Some(r) => {
            r.x = p.position.x as i32;
            r.y = p.position.y as i32;
          },
          None => {},
        }
      },
      None => {},
    }
    
  }
}