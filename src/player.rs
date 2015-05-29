use sprite::Sprite;

//constants
use assets::P_ACCEL;
use assets::P_MAX_SPEED;

///Player is the struct that keeps track of the player
pub struct Player {
  pub sprite: Sprite,
  pub status: PlayerStatus,
  pub v_x: i32,
  pub v_y: i32,
}

///The player can take on one of several statuses
pub enum PlayerStatus {
  Stationary,
  MovingLeft,
  MovingRight,
  Decelerating,
}

impl Player {
  ///Creates a new Player
  pub fn new(spr: Sprite, sta: PlayerStatus) -> Player {
    Player {sprite: spr, status: sta, v_x: 0, v_y: 0}
  }
  ///Updates the player based on internal state
  pub fn update(&mut self) {
    match self.status {
      PlayerStatus::MovingLeft => {
        //player.sprite.mov_x(-1 * P_SPEED);
        if self.v_x > -1 * P_MAX_SPEED {
          self.v_x -= P_ACCEL;
        }
      },
      PlayerStatus::MovingRight => {
        //player.sprite.mov_x(P_SPEED);
        if self.v_x < P_MAX_SPEED {
          self.v_x += P_ACCEL;
        }
      },
      PlayerStatus::Decelerating => {
        if self.v_x > 0 {
          self.v_x -= P_ACCEL;
        }
        else if self.v_x < 0 {
          self.v_x += P_ACCEL;
        }
        else {
          self.status = PlayerStatus::Stationary;
        }
      }
      _ => {}
    }
    self.sprite.mov(self.v_x,self.v_y);
  }
}