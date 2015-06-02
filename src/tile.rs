use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Tile {
  tile_x: i32,
  tile_y: i32,
  tile_type: TileEnum,
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
  pub fn debug_new_tile(x: i32, y: i32) -> Tile {
    return Tile{ tile_x: x, tile_y: y, tile_type: TileEnum::Floor, pass_up: false,
      pass_right: false, pass_left: false, pass_down: false};
  }
}