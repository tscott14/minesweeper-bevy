mod cell;
mod field;
mod minesweeper;

use bevy::{core::FixedTimestep, prelude::*};

fn welcome() {
    println!("Welcome!\nLeft click to uncover cell.\nRight click to flag cell.\nPress Space Bar to Reset Game. \nCurrent flag count will be printed in console.");
}

fn main() {
    let field = field::Field::new(30, 16, 40, 100);
    let width = (field.dimensions.width * field.cell_size) as f32;
    let height = (field.dimensions.height * field.cell_size) as f32;

    welcome();

    App::new()
        .insert_resource(WindowDescriptor {
            width,
            height,
            resizable: false,
            title: String::from("Minesweeper-Bevy"),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(minesweeper::GameState::new())
        .insert_resource(field)
        .insert_resource(minesweeper::FlagCount::new(100))
        .add_startup_system(minesweeper::setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 20.0))
                .with_system(minesweeper::update_field),
        )
        .add_system(minesweeper::check_cell_selected)
        .add_system(minesweeper::check_for_game_reset)
        .add_system(minesweeper::check_for_win)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
