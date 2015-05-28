extern crate sdl2;

use sdl2::render::RenderDrawer;

use sprite::Sprite;

pub trait Draw {
  fn draw(&self, d: &mut RenderDrawer);
}

impl Draw for Sprite {
  fn draw(&self, d: &mut RenderDrawer) {
    d.copy_ex(&self.tex, None, self.rect
      , self.angle, None, (false,false));
  }
}