//file for managing maps of tiles
use std::vec::Vec;
use sdl2::render::Renderer;

use tile::Tile;

///Type synonym for vectors of tiles, which are maps
pub struct TileMap(Vec<Tile>);

impl TileMap {
  pub fn unwrap(&self) -> &Vec<Tile> {
    let TileMap(ref result) = *self;
    result
  }
}

pub fn debug_tilemap(r: &Renderer) -> Vec<Tile> {
  let tile1 = Tile::debug_new_tile(20,10, r);
  let tile2 = Tile::debug_new_tile(21,10, r);
  let tile3 = Tile::debug_new_tile(22,10, r);
  vec![tile1, tile2, tile3]
}