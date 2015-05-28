extern crate sdl2;

use sdl2::render::RenderDrawer;

use sprite::Sprite;
use player::Player;

pub trait Draw {
  fn draw(&self, d: &mut RenderDrawer);
}

impl Draw for Sprite {
  fn draw(&self, d: &mut RenderDrawer) {
    d.copy_ex(&self.tex, None, self.rect
      , self.angle, None, (false,false));
  }
}

impl Draw for Player {
  fn draw(&self, d: &mut RenderDrawer) {
    self.sprite.draw(d);
  }
}