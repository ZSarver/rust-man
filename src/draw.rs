use sdl2::render::RenderDrawer;
use sdl2::render::Texture;
use sdl2::rect::Rect;

use sprite::Sprite;
use player::Player;
use tile::Tile;
use assets::TILE_SIZE;

pub trait Draw {
  fn draw(&self, d: &mut RenderDrawer, t: &Option<Texture>);
}

impl Draw for Sprite {
  fn draw(&self, d: &mut RenderDrawer, t: &Option<Texture>) {
    d.copy_ex(&self.tex, None, self.rect
      , self.angle, None, (false,false));
  }
}

impl Draw for Player {
  fn draw(&self, d: &mut RenderDrawer, t: &Option<Texture>) {
    self.sprite.draw(d, &None);
  }
}

impl Draw for Tile {
  fn draw(&self, d: &mut RenderDrawer, t: &Option<Texture>) {
    match *t {
      Some(ref tex) => {
        let r = Some(Rect::new(TILE_SIZE * self.tile_x,
          TILE_SIZE * self.tile_y,
          TILE_SIZE,
          TILE_SIZE));
        d.copy_ex(t.as_ref().unwrap(), None, r, 0.0, None, (false, false));
      }
      None => {
        panic!("No texture given for tile drawing");
      }
    }
  }
}