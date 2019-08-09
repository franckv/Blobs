use std::collections::HashSet;

use amethyst::core::{Hidden, Transform};
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System} ;

use crate::config::BlobsConfig;
use crate::utils::geometry;
use crate::map::Map;
use crate::components::{Explored, Init, Intent, Mob, Player, Tile};

#[derive(Default)]
pub struct FovSystem;

impl<'s> System<'s> for FovSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Init>,
        ReadStorage<'s, Intent>,
        ReadStorage<'s, Mob>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Explored>,
        ReadStorage<'s, Hidden>,
        Read<'s, LazyUpdate>,
        Read<'s, Map>,
        Read<'s, BlobsConfig>,
        Entities<'s>
    );

    fn run(&mut self, (transform, init, intents, mob, player, tiles, explored,
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
            let fov = generate_fov(player_x, player_y,
                               map.width(), map.height(),
                               &map, &tiles, config.fov.radius);

            for (_, _, transform, entity) in (
                &tiles, !&explored, &transform, &entities).join() {
                let x = transform.translation().x as usize;
                let y = transform.translation().y as usize;
                if fov.contains(&(x, y)) {
                    if let Some(_) = hidden.get(entity) {
                        update.remove::<Hidden>(entity);
                    }
                    update.insert(entity, Explored);
                } else {
                    update.insert::<Hidden>(entity, Hidden);
                }
            }

            for (_, transform, entity) in (&mob, &transform, &entities).join() {
                let x = transform.translation().x as usize;
                let y = transform.translation().y as usize;
                if fov.contains(&(x, y)) {
                    if let Some(_) = hidden.get(entity) {
                        update.remove::<Hidden>(entity);
                    }
                } else {
                    update.insert::<Hidden>(entity, Hidden);
                }
            }
        }
    }
}

fn generate_fov<'s>(player_x: f32, player_y: f32,
                    width: usize, height: usize,
                    map: &Map, tiles: &ReadStorage<'s, Tile>,
                    radius: usize) -> HashSet<(usize, usize)> {
    let mut fov = HashSet::new();

    geometry::draw_circle(
        player_x as i32, player_y as i32,
        radius as i32, width as i32, height as i32, |bx, by| {
            geometry::draw_line(
                player_x as i32, player_y as i32,
                bx, by, |px, py| {
                    let entity = map.tile(px as usize, py as usize);
                    let tile = tiles.get(entity).unwrap();

                    if !tile.is_transparent() {
                        fov.insert((px as usize, py as usize));
                        return false;
                    }

                    fov.insert((px as usize, py as usize));
                    true
                });

            true
        });

    fov
}

