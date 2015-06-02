use rustc_serialize::json;
use sdl2::render::Texture;
use sdl2::render::Renderer;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use std::path::Path;

use assets::TILE_SIZE;
use assets::ASSET_STR;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Tile {
  pub tile_x: i32,
  pub tile_y: i32,
  tile_type: TileEnum,
//  tile_rect: Option<Rect>, can't have a Rect with Rustc*odable
  pass_up: bool,
  pass_left: bool,
  pass_right: bool,
  pass_down: bool,
}

#[derive(RustcDecodable, RustcEncodable)]
enum TileEnum {
  Ladder,
  Floor,
  Prisoner,
}

impl Tile {
  pub fn debug_new_tile(x: i32, y: i32, r: &Renderer) -> Tile {
    let asset_path = Path::new(ASSET_STR);
    let full_pathbuf = asset_path.join("sonic.bmp");
    let full_path = full_pathbuf.as_path();
    
    //load surface
    let surf = Surface::from_bmp(full_path)
      .ok().expect("Failed to load asset.");
    
    //create a texture from the surface
    let texture = r.create_texture_from_surface(&surf)
      .ok().expect("Failed to create texture");
    return Tile{ tile_x: x,
      tile_y: y,
      tile_type: TileEnum::Floor,
      //tile_rect: Some(Rect::new(TILE_SIZE * x, TILE_SIZE * y, TILE_SIZE, TILE_SIZE)),
      pass_up: false,
      pass_right: false,
      pass_left: false,
      pass_down: false};
  }
}