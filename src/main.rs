use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;

pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();

        player_input(self, context);
        self.run_system();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, context);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            context.set(
                pos.x,
                pos.y,
                render.foreground,
                render.background,
                render.glyph,
            );
        }
    }
}

impl State {
    fn run_system(&mut self) {
        self.ecs.maintain();
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();
    let mut game_state = State { ecs: World::new() };
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    game_state.ecs.insert(map);

    game_state
        .ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, game_state);
}
