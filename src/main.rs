use macroquad::prelude::*;
//use macroquad_platformer::*;

mod player;
use player::*;

mod world;
use world::*;

fn initPlayers(map: &mut MapWorld)
{
    map.players.push(Player {
        collider: map.world.add_actor(vec2(50.0, 50.0), tile_size as i32, tile_size as i32),
        speed: vec2(0., 0.),
        up: KeyCode::Up,
        down: KeyCode::Down,
        left: KeyCode::Left,
        right: KeyCode::Right,
        sprite: 5,
    });

    map.players.push(Player {
        collider: map.world.add_actor(vec2(100.0, 50.0), tile_size as i32, tile_size as i32),
        speed: vec2(0., 0.),
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,
        sprite: 101,
    });
}

#[macroquad::main("InputKeys")]
async fn main() {
    //let mut x: f32 = screen_width() / 2.0;
    //let mut y = screen_height() / 2.0;

    let mut gamemap = MapWorld::new("resources/tileset.png", "resources/map.json", "tileset".to_string());

    //addPlayer(&mut gamemap, 101);
    initPlayers(&mut gamemap);
    
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, tile_size * map_height, tile_size * map_width / 2., -1. * tile_size * map_height));
    let oldPos = camera.target.x;
    //camera.rotation = 5.;
 
    loop {
        clear_background(LIGHTGRAY);
        let pos = gamemap.world.actor_pos(gamemap.players[1].collider);

        //if(pos.x > oldPos)
        {
            camera.target.x = pos.x ;
        }

        set_camera(&camera);

        gamemap.drawWorld();
        gamemap.checkCollide();

        //draw_circle(pos.x, pos.y, 15.0, YELLOW);

        next_frame().await
    }
}



/*

struct Platform {
    collider: Solid,
    speed: f32,
}


    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 8., 8., 40, 1);



    let mut platform = Platform {
        collider: world.add_solid(vec2(170.0, 130.0), 32, 8),
        speed: 50.,
    };


        // draw platform
        {
            let pos = world.solid_pos(platform.collider);
            tiled_map.spr_ex(
                "tileset",
                Rect::new(6.0 * 8.0, 0.0, 32.0, 8.0),
                Rect::new(pos.x, pos.y, 32.0, 8.0),
            )
        }



        // platform movement
        {
            world.solid_move(platform.collider, platform.speed * get_frame_time(), 0.0);
            let pos = world.solid_pos(platform.collider);
            if platform.speed > 1. && pos.x >= 220. {
                platform.speed *= -1.;
            }
            if platform.speed < -1. && pos.x <= 150. {
                platform.speed *= -1.;
            }
        }

*/