use macroquad::prelude::*;
use macroquad_platformer::*;
use pub_fields::pub_fields;

#[pub_fields]
pub struct Player {
    collider: Actor,
    speed: Vec2,
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
    sprite: u32,
}

impl Player{
    pub fn calcSpeed(&mut self)
    {
        if is_key_down(self.right) {
            self.speed.x += 10.0;
        }
        if is_key_down(self.left) {
            self.speed.x -= 10.0;
        }
        if is_key_down(self.down) {
            self.speed.y += 10.0;
        }
        if is_key_down(self.up) {
            self.speed.y -= 10.0;
        }
    }

}