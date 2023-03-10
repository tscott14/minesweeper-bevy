use bevy::app::AppExit;
use bevy::prelude::*;

use crate::cell::{CellState, CellType};
use crate::coordinates::Coordinates;
use crate::field::Field;

#[derive(PartialEq)]
enum GameStates {
    Playing,
    GameOver,
    GameWon,
}

pub struct GameState {
    game_state: GameStates,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            game_state: GameStates::Playing,
        }
    }
}
pub struct FlagCount {
    refill_count: usize,
    count: usize,
}

impl FlagCount {
    pub fn new(count: usize) -> Self {
        Self {
            refill_count: count,
            count,
        }
    }
}

#[derive(Component)]
pub struct FieldButton;

pub fn setup(
    mut commands: Commands,
    map: Res<Field>,
    window: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    let window = window.get_primary().unwrap();
    let cell_size = map.cell_size as f32;
    let half_width = window.width() * 0.5f32;
    let half_height = window.height() * 0.5f32;
    let half_cell_size = cell_size * 0.5f32;
    let cell_scale = 0.96;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let text_style = TextStyle {
        font: asset_server.load("fonts/NotoSansMono-Regular.ttf"),
        font_size: cell_size,
        color: Color::BLACK,
    };

    for y in 0..map.dimensions.get_height() {
        for x in 0..map.dimensions.get_width() {
            let loc_x = (x as f32) * cell_size - half_width + half_cell_size;
            let loc_y = (y as f32) * cell_size - half_height + half_cell_size;

            commands
                .spawn()
                .insert(FieldButton)
                .insert(Button)
                .insert(Coordinates {
                    x_coord: x as isize,
                    y_coord: y as isize,
                })
                .insert_bundle(SpriteBundle {
                    //materials: materials.add(Color::rgb(1.0, 0.7, 0.7).into()),
                    sprite: Sprite {
                        color: Color::rgb(0.777f32, 0.777f32, 0.777f32),

                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(loc_x, loc_y, 0.0),
                        scale: Vec3::new(cell_size * cell_scale, cell_size * cell_scale, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });

            let text_left = Val::Px(x as f32 * cell_size + (cell_size / 4.));
            let text_top = Val::Px(y as f32 * cell_size);
            commands
                .spawn()
                .insert(FieldButton)
                .insert(Coordinates {
                    x_coord: x as isize,
                    y_coord: y as isize,
                })
                .insert_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: String::from(""),
                            style: text_style.clone(),
                        }],
                        ..Default::default()
                    },
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: text_top,
                            left: text_left,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                });
        }
    }
}

pub fn update_field(
    mut map: ResMut<Field>,
    mut field_entities: Query<(&Coordinates, &mut Sprite), With<FieldButton>>,
    mut field_entities_bomb_counter_text: Query<(&Coordinates, &mut Text), With<FieldButton>>,
    game_state: Res<GameState>,
) {
    let color_unknown = Color::rgb(0.777, 0.777, 0.777);
    let color_safe = Color::rgb(0.555, 0.555, 0.555);
    let color_wise = Color::rgb(0.111, 0.555, 0.111);
    let color_alert = Color::rgb(0.444, 0.000, 0.444);
    let color_critical = Color::rgb(1.0, 0.0, 0.0);
    let color_dead = Color::rgb(0.0, 0.0, 0.0);

    for (coords, mut sprite) in field_entities.iter_mut() {
        // This should never assert!
        sprite.color = match map.get_mut(&coords).unwrap() {
            (CellType::EMPTY(_), CellState::UNKNOWN) => color_unknown,
            (CellType::EMPTY(_), CellState::FLAGGED) => color_alert,
            (CellType::EMPTY(_), CellState::EXPOSED) => color_safe,
            (CellType::BOMB, CellState::EXPOSED) => color_critical,
            (CellType::BOMB, CellState::FLAGGED) => {
                if game_state.game_state == GameStates::GameOver
                    || game_state.game_state == GameStates::GameWon
                {
                    color_wise
                } else {
                    color_alert
                }
            }
            (CellType::BOMB, CellState::UNKNOWN) => {
                if game_state.game_state == GameStates::GameOver {
                    color_dead
                } else {
                    color_unknown
                }
            }
        };
    }

    for (coords, mut text) in field_entities_bomb_counter_text.iter_mut() {
        if let (CellType::EMPTY(Some(bomb_count)), _) = map.get_clone(&coords).unwrap() {
            text.sections[0].value = format!("{}", bomb_count);
        }
    }
}

fn flood_fill(mut field: &mut ResMut<Field>, coords: &Coordinates) {
    let bombs = field.count_bomb_neighbors(coords);
    if bombs > 0 {
        field.set_cell_type(&coords, CellType::EMPTY(Some(bombs)));
        field.set_cell_state(&coords, CellState::EXPOSED);
        return;
    }

    let x = coords.x_coord;
    let y = coords.y_coord;
    match field.get_clone(&coords) {
        Some((CellType::EMPTY(_), CellState::UNKNOWN)) => {
            field.set_cell_state(&coords, CellState::EXPOSED);

            flood_fill(&mut field, &Coordinates::new(x - 1, y));
            flood_fill(&mut field, &Coordinates::new(x + 1, y));
            flood_fill(&mut field, &Coordinates::new(x, y - 1));
            flood_fill(&mut field, &Coordinates::new(x, y + 1));
        }
        _ => {}
    }
}

fn process_cell(
    coords: Coordinates,
    mut field: &mut ResMut<Field>,
    game_state: &mut ResMut<GameState>,
) {
    let cell = field.get_mut(&coords).unwrap();
    match cell {
        (_, CellState::FLAGGED) => {} // Do Nothing
        (_, CellState::EXPOSED) => {} // Do Nothing
        (CellType::EMPTY(_), CellState::UNKNOWN) => {
            // flood fill
            flood_fill(&mut field, &coords);
        }

        (CellType::BOMB, CellState::UNKNOWN) => {
            // game over
            game_state.game_state = GameStates::GameOver;
            println!("Game Over");
        }
    }
}

pub fn check_cell_selected(
    mut field: ResMut<Field>,
    mut game_state: ResMut<GameState>,
    mut flag_count: ResMut<FlagCount>,
    btns: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if game_state.game_state == GameStates::GameOver {
        return;
    }

    if game_state.game_state == GameStates::GameWon {
        return;
    }

    if btns.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        let cell_size = field.cell_size as f32;
        if let Some(position) = window.cursor_position() {
            let x_coord = (position.x / cell_size) as isize;
            let y_coord = (position.y / cell_size) as isize;
            process_cell(
                Coordinates { x_coord, y_coord },
                &mut field,
                &mut game_state,
            );
        }
    }

    if btns.just_pressed(MouseButton::Right) {
        let window = windows.get_primary().unwrap();
        let cell_size = field.cell_size as f32;
        if let Some(position) = window.cursor_position() {
            let x_coord = (position.x / cell_size) as isize;
            let y_coord = (position.y / cell_size) as isize;

            if let Some((_, state)) = field.get_clone(&Coordinates { x_coord, y_coord }) {
                if state == CellState::FLAGGED {
                    flag_count.count += 1;
                    println!("Flags Remaining: {}", flag_count.count);
                    field.set_cell_state(&Coordinates { x_coord, y_coord }, CellState::UNKNOWN);
                } else {
                    if flag_count.count <= 0 {
                        println!(
                            "You do not have any remaining flags! Reclaim the ones on the field!"
                        );
                    } else {
                        flag_count.count -= 1;
                        println!("Flags Remaining: {}", flag_count.count);
                        field.set_cell_state(&Coordinates { x_coord, y_coord }, CellState::FLAGGED);
                    }
                }
            }
        }
    }
}

pub fn check_for_win(
    mut game_state: ResMut<GameState>,
    mut exit: EventWriter<AppExit>,
    field: Res<Field>,
) {
    for (cell_type, cell_state) in field.cells.iter() {
        if cell_type == &CellType::BOMB && cell_state != &CellState::FLAGGED {
            return;
        }
    }
    game_state.game_state = GameStates::GameWon;
    println!("Congrats, You won");
    exit.send(AppExit);
}

pub fn check_for_game_reset(
    mut field: ResMut<Field>,
    mut game_state: ResMut<GameState>,
    mut flag_count: ResMut<FlagCount>,
    mut field_entities_bomb_count: Query<&mut Text, With<FieldButton>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        field.reset();
        game_state.game_state = GameStates::Playing;
        flag_count.count = flag_count.refill_count;

        for mut text in field_entities_bomb_count.iter_mut() {
            text.sections[0].value = String::from("");
        }
    }
}
