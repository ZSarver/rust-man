use sprite::Sprite;

pub struct Player {
  pub sprite: Sprite,
  pub status: PlayerStatus,
}

pub enum PlayerStatus {
  Stationary,
  MovingLeft,
  MovingRight,
}

impl Player {
  pub fn new(spr: Sprite, sta: PlayerStatus) -> Player {
    Player {sprite: spr, status: sta}
  }
}