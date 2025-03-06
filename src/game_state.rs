use bracket_lib::{
    color::RGB,
    prelude::{self as rltk, GameState},
};
use specs::prelude::*;

use crate::components::{Player, Position, Renderable};
use crate::map::*;
use crate::player::*;

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn new() -> Self {
        let mut gs = State { ecs: World::new() };
        gs.ecs.register::<Position>();
        gs.ecs.register::<Renderable>();
        gs.ecs.register::<Player>();

        let (rooms, map) = new_map_rooms_and_corridors();
        gs.ecs.insert(map);
        let (player_x, player_y) = rooms[0].center();

        gs.ecs
            .create_entity()
            .with(Position {
                x: player_x,
                y: player_y,
            })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player {})
            .build();

        gs
    }

    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}
