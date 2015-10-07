//file for managing maps of tiles
use std::vec::Vec;
use sdl2::render::Renderer;

use tile::Tile;
use tile::SaveTile;

///Type synonym for vectors of tiles, which are maps
pub struct TileMap(Vec<Tile>);

///Simplified TileMap for saving to disk
#[derive(RustcDecodable, RustcEncodable)]
pub struct SaveTileMap(Vec<SaveTile>);

impl TileMap {
  pub fn unwrap(&self) -> &Vec<Tile> {
    let TileMap(ref result) = *self;
    result
  }
}

impl SaveTileMap {
  pub fn to_TileMap(&self, r: &Renderer) -> TileMap {
    let SaveTileMap(ref innards) = *self;
    let mut tiles = Vec::new();
    for t in innards {
      tiles.push(t.to_Tile(r));
    }
    TileMap(tiles)
  }
}

pub fn debug_tilemap(r: &Renderer) -> Vec<Tile> {
  let tile1 = Tile::debug_new_tile(20,10, r);
  let tile2 = Tile::debug_new_tile(21,10, r);
  let tile3 = Tile::debug_new_tile(22,10, r);
  vec![tile1, tile2, tile3]
}
