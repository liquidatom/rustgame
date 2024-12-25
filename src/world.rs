use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;
use pub_fields::pub_fields;
use futures::executor;

use crate::player::*;

#[pub_fields]
pub struct MapWorld
{
    tiled_map: tiled::Map,
    tilesetName: String, 
    world: World,
    players: Vec<Player>,
}

pub const tile_size: f32 = 32.;
pub const map_width: f32 = 60.;
pub const map_height: f32 = 20.;

impl MapWorld
{
    pub fn new(tilesetFile: &str, mapFile: &str, tilesetName: String) -> MapWorld
    {
        let tileset = executor::block_on(load_texture(tilesetFile)).unwrap();
        tileset.set_filter(FilterMode::Nearest);

        let tiled_map_json = executor::block_on(load_string(mapFile)).unwrap();
        let tiled_map = tiled::load_map(&tiled_map_json, &[(tilesetFile, tileset)], &[]).unwrap();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("main", None) {
            static_colliders.push(if tile.is_some() {
                Tile::Solid
            } else {
                Tile::Empty
            });
        }
    
        let mut world = World::new();
        world.add_static_tiled_layer(static_colliders, tile_size, tile_size, map_width as usize, 1);
    
        MapWorld{tiled_map, tilesetName, world, players: Vec::new()}
    }

    pub fn drawWorld(&self) -> ()
    {
        self.tiled_map.draw_tiles("main", Rect::new(0.0, 0.0, tile_size * map_width, tile_size * map_height), None);

        for player in self.players.iter()
        {
            let pos = self.world.actor_pos(player.collider);
            self.tiled_map.spr(&self.tilesetName, player.sprite, Rect::new(pos.x, pos.y, tile_size, tile_size));
        }
        
    }

    pub fn checkCollide(&mut self)
    {
        for player in self.players.iter_mut()
        {
            player.calcSpeed();
            let pos = self.world.actor_pos(player.collider);
            let mut on_ground = self.world.collide_check(player.collider, pos + vec2(0., 1.));

            println!("== {on_ground}, x = {}, y = {}", pos.x, pos.y);

            if on_ground
            {
                player.speed.y = 0.0;
            }

            self.world.move_h(player.collider, player.speed.x * get_frame_time());
            self.world.move_v(player.collider, player.speed.y * get_frame_time());
        }
    }

} 