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