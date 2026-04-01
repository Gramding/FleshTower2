use super::{Fov, Map, Player, Position};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct FOV {}

impl<'a> System<'a> for FOV {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Fov>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut fov, pos, player) = data;

        for (ent, fov, pos) in (&entities, &mut fov, &pos).join() {
            if fov.dirty {
                fov.dirty = false;
                fov.visible_tiles.clear();
                fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
                fov.visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in fov.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
