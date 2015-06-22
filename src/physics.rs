use std::ops::Add;
use std::ops::Mul;

use sprite::Sprite;

///2-vectors
#[derive(Clone)]
pub struct Vec2 {
  x: f64,
  y: f64,
}

impl Vec2 {
  pub fn new(nx: f64, ny: f64) -> Vec2 {
    Vec2 { x: nx, y: ny }
  }
}

impl Add for Vec2 {
  type Output = Vec2;
  
  fn add(self, other: Vec2) -> Vec2{
    Vec2 { x: self.x + other.x, y: self.y + other.y }
  }
}

//scalar multiplication... on the right
impl Mul<f64> for Vec2 {
  type Output = Vec2;
  
  fn mul(self, scalar: f64) -> Vec2 {
    Vec2 { x: self.x * scalar, y: self.y * scalar}
  }
}

///Physics object
pub struct Phobject {
  position: Vec2,
  velocity: Vec2,
  accel: Vec2,
  mass: f64,
  ///terminal speed
  term_speed: f64,
}

impl Phobject {
  pub fn new(pos: Vec2, vel: Vec2, acc: Vec2, m: f64, ts: f64) -> Phobject {
    Phobject { position: pos, velocity: vel, accel: acc, mass: m, term_speed: ts }
  }
  ///Update the Phobject
  ///
  ///dt is the time in milliseconds
  pub fn update(&mut self, dt: f64) {
    self.velocity.x = self.velocity.x + (dt/1000.0) * self.accel.x;
    self.velocity.y = self.velocity.y + (dt/1000.0) * self.accel.y;
    
    //reduce to terminal speed
    if self.speed() > self.term_speed {
      let factor = self.term_speed / self.speed();
      self.velocity = self.velocity.clone() * factor;
    }
  }
  pub fn speed(&self) -> f64 {
    (self.velocity.x.powi(2) + self.velocity.y.powi(2)).abs()
  }
  pub fn apply_force(&mut self, f: Vec2) {
    self.accel = self.accel.clone() + f * (1.0/self.mass);
  }
}