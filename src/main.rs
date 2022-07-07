mod coordinates;
mod dimensions;
mod cell;
mod field;
mod minesweeper;

use bevy::{core::FixedTimestep, prelude::*};

const FIXED_UPDATES_PER_SECOND: f64 = 20.0f64;

fn print_welcome_message() {
    println!("Welcome!");
    println!("Left click to uncover cell.");
    println!("Right click to flag cell.");
    println!("Press Space Bar to Reset Game.");
    println!("Current flag count will be printed in console.");
    println!("You win by flagging all the bombs!");
}

fn main() {
    let field = field::Field::new(9, 9, 40, 10);
    let width = (field.dimensions.get_width() * field.cell_size) as f32;
    let height = (field.dimensions.get_height() * field.cell_size) as f32;

    print_welcome_message();
 
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
        .insert_resource(minesweeper::FlagCount::new(field.bomb_count))
        .insert_resource(field)
        .add_startup_system(minesweeper::setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0f64 / FIXED_UPDATES_PER_SECOND))
                .with_system(minesweeper::update_field),
        )
        .add_system(minesweeper::check_cell_selected)
        .add_system(minesweeper::check_for_game_reset)
        .add_system(minesweeper::check_for_win)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
