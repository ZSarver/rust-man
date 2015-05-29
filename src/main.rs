extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::keycode::KeyCode;
use sdl2::event::Event;
use sdl2::timer::get_ticks;

mod sprite;
mod assets;
mod draw;
mod player;

use sprite::Sprite;
use player::Player;
use player::PlayerStatus;
use draw::Draw;

//constants
use assets::P_ACCEL;
use assets::P_MAX_SPEED;

fn main() {
  //initialize sdl
  let sdl_context = sdl2::init().video().events().build()
    .ok().expect("Failed to initialize SDL.");
  
  //create a window
  let window = sdl_context.window("Rust-Man", 800, 600)
    .position_centered()
    .build()
    .ok().expect("Failed to create window.");
  
  //create a renderer
  let mut renderer = window.renderer().accelerated().build()
    .ok().expect("Failed to create accelerated renderer.");
    
  //create a new player
  let mut player = Player::new(Sprite::new_from_file("sonic.bmp", &renderer),
    PlayerStatus::Stationary);
    
  //start drawing
  let mut drawer = renderer.drawer();
  drawer.clear();
  drawer.present();
  
  //event loop stuff
  let mut running = true;
  let mut event_pump = sdl_context.event_pump();
  let mut prev_time = get_ticks();
  let mut delta_t = get_ticks() - prev_time;
    
  while running {
    //timer stuff
    delta_t = get_ticks() - prev_time;
    
    //limit to 60fps
    if delta_t > 16 {
      //handle event queue
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit {..} | Event::KeyDown {keycode: KeyCode::Escape, .. } => {
            running = false
          },
          Event::KeyDown {keycode: KeyCode::Left, .. } => {
            player.status = PlayerStatus::MovingLeft
          },
          Event::KeyDown {keycode: KeyCode::Right, .. } => {
            player.status = PlayerStatus::MovingRight
          },
          Event::KeyUp {keycode: KeyCode::Left, .. } => {
            player.status = PlayerStatus::Decelerating
          },
          Event::KeyUp {keycode: KeyCode::Right, .. } => {
            player.status = PlayerStatus::Decelerating
          },
          _ => {}
        }
      }
      
      //move player      
      player.update();
      
      //draw
      drawer.clear();
      player.draw(&mut drawer);
      drawer.present();
      
      //more timer stuff
      prev_time = get_ticks();
    }
  }
  
  println!("Hello, world!");
}
