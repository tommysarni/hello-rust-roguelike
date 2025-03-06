use crate::components::{Player, Position};
use crate::game_state::{xy_idx, State};
use crate::map::TileType;
use bracket_lib::prelude::{self as rltk};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut rltk::BTerm) {
    // Player movement
    if let Some(key) = ctx.key {
        use bracket_lib::prelude::VirtualKeyCode::*;
        let delta = match key {
            Left | Key4 | H => (-1, 0),
            Right | Key6 | L => (1, 0),
            Up | Key8 | K => (0, -1),
            Down | Key2 | J => (0, 1),
            _ => (0, 0),
        };

        if delta != (0, 0) {
            try_move_player(delta.0, delta.1, &mut gs.ecs);
        }
    }
}
