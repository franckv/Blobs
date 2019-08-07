use amethyst::core::{Hidden, Transform};
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System} ;

use crate::config::FovConfig;
use crate::map::Map;
use crate::components::{Explored, Init, Intent, Player, Tile};

#[derive(Default)]
pub struct FovSystem;

impl<'s> System<'s> for FovSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Init>,
        ReadStorage<'s, Intent>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Explored>,
        ReadStorage<'s, Hidden>,
        Read<'s, LazyUpdate>,
        Read<'s, Map>,
        Read<'s, FovConfig>,
        Entities<'s>
    );

    fn run(&mut self, (transform, init, intents, player, tiles, explored,
                       hidden, update, map, config, entities): Self::SystemData) {
        let (player_x, player_y, compute) = {
            let (mut player_x, mut player_y) = (0., 0.);

            let mut compute = {
                let mut is_init = false;
                for _ in init.join() {
                    is_init = true;
                    break;
                }

                is_init
            };

            if compute {
                for (transform, _) in (&transform, &player).join() {
                    player_x = transform.translation().x;
                    player_y = transform.translation().y;
                    break;
                }
            } else {
                for (transform, _, _) in (&transform, &intents, &player).join() {
                    player_x = transform.translation().x;
                    player_y = transform.translation().y;
                    compute = true;
                    break;
                }
            }

            (player_x, player_y, compute)
        };


        if compute {
            for (transform, _, _, entity) in (&transform, &tiles, !&explored, &entities).join() {
                let (tile_x, tile_y) =
                    (transform.translation().x, transform.translation().y);

                if is_in_fov(tile_x, tile_y, player_x, player_y, &config) {
                    if let Some(_) = hidden.get(entity) {
                        update.remove::<Hidden>(entity);
                    }
                    update.insert(entity, Explored);
                } else {
                    update.insert::<Hidden>(entity, Hidden);
                }
            }
        }
    }
}

fn is_in_fov(tile_x: f32, tile_y: f32,
             player_x: f32, player_y: f32,
             config: &FovConfig) -> bool {
    let radius = config.radius as f32;
    let distance = config.distance;

    let d = match distance {
        0 => {
            manhattan_distance(tile_x, tile_y, player_x, player_y)
        },
        1 => {
            diagonal_distance(tile_x, tile_y, player_x, player_y)
        },
        _ => {
            circle_distance(tile_x, tile_y, player_x, player_y)
        }
    };

    if d < radius {
        true
    } else {
        false
    }
}

fn manhattan_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn diagonal_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x1 - x2).abs().max((y1 - y2).abs())
}

fn circle_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (((x1 - x2) * (x1 - x2)) + ((y1 - y2) * (y1 - y2))).sqrt()
}
