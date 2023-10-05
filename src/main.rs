use std::{ops::Div, collections::VecDeque};
use bevy::window::{PresentMode, WindowResolution};

use bevy::prelude::*;

use bevy_egui::{egui::{self, FontDefinitions, FontFamily, FontData}, EguiContexts, EguiPlugin};

#[derive(Resource, Clone)]
struct Fields {
    cells: [[Option<Entity>; FIELD_WIDTH]; FIELD_HEIGHT],
}

impl Fields {
    fn get_cell(&self, x: isize, y: isize) -> Option<Entity> {
        if x < 0 {return None;}
        if y < 0 {return None;}
        if x == (FIELD_WIDTH as isize) {return None;}
        if y == (FIELD_HEIGHT as isize) {return None;}
        self.cells[x as usize][y as usize]
    }

    fn get_x(&self, x: usize) -> f32 {
        (CELL_SIZE * x) as f32 + (CELL_SIZE-1).div(2)  as f32 - (CELL_SIZE * FIELD_WIDTH.div(2)) as f32
    }

    fn get_y(&self, y: usize) -> f32 {
        (CELL_SIZE * y)  as f32+ (CELL_SIZE-1).div(2)  as f32 - (CELL_SIZE * FIELD_HEIGHT.div(2)) as f32
    }
}


const FIELD_WIDTH: usize = 100;
const FIELD_HEIGHT: usize = 100;

const CELL_SIZE: usize = 15;


impl Default for Fields {
    fn default() -> Self {
        Fields {
            cells: [[None; FIELD_WIDTH]; FIELD_HEIGHT],
        }
    }
}


#[derive(Resource)]
struct Config {
    interval_ms: u64,
    alive_rule: Rule,
    dead_rule: Rule
}


impl Default for Config {
    fn default() -> Self {
        Config {
            interval_ms: 10,
            alive_rule: Rule {
                min: 2,
                max: 3
            },
            dead_rule: Rule {
                min: 3,
                max: 3
            }
        }
    }
}

struct Rule {
    min: u8,
    max: u8
}

impl Rule {
    fn will_spawn(&self, count: u8) -> bool {
        count >= self.min && count <= self.max
    }
}

#[derive(Resource)]
struct WorldTimer(Timer);


#[derive(Component, Debug, PartialEq, Eq)]
struct Cell {
    pub is_alive: bool
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            is_alive: false
        }
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(".lifegame".to_string()),
                title: "I am a window!".to_string(),
                present_mode: PresentMode::AutoVsync,
                resolution: WindowResolution::new(1000., 600.),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_resource::<Fields>()
        .init_resource::<Config>()
        .add_systems(Startup, (setup, ))
        .add_systems(PostStartup, set_default_alived_cell)
        // initialize resources
        .add_systems(Update, (
            move_forward,
            ui_example_system,
            display_cells
        ))
        .run();
}


fn setup(mut commands: Commands, _gizmos: Gizmos, config: Res<Config>, mut fields: ResMut<Fields>, mut egui_contexts: EguiContexts) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(WorldTimer(Timer::from_seconds(config.interval_ms as f32 /1000., TimerMode::Once)));

     // loop through all cells
     for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            let id = commands.spawn(
                (
                    Cell::default(),
                    SpriteBundle {
                        visibility: Visibility::Visible,
                        sprite: Sprite {
                            color: Color::rgb(1., 0.0, 0.),
                            custom_size: Some(Vec2::splat(CELL_SIZE as f32)-1.),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(fields.get_x(x) as f32, fields.get_y(y)as f32, 0.)),
                        ..default()
                    },
                )
            ).id();

            fields.cells[x][y] = Some(id);
        }
    }


    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        FontData::from_static(include_bytes!("../NotoSansJP-Regular.ttf")),
    );
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
    egui_contexts.ctx_mut().set_fonts(fonts);

}


fn set_default_alived_cell(mut fields: ResMut<Fields>,mut query: Query<&mut Cell>) {
    set_pattern_random(&mut fields,&mut query);
}

fn set_pattern_blinker(fields: &mut ResMut<Fields>,query: &mut Query<&mut Cell>) {

    query.iter_mut().for_each(|mut cell| {
        cell.is_alive = false;
    });

    let base_x = (FIELD_WIDTH as f32/2.).floor() as isize;
    let base_y = (FIELD_HEIGHT as f32/2.).floor() as isize;

    query.get_component_mut::<Cell>(fields.get_cell(base_x,base_y).unwrap()).unwrap().is_alive = true;
    query.get_component_mut::<Cell>(fields.get_cell(base_x+1,base_y).unwrap()).unwrap().is_alive = true;
    query.get_component_mut::<Cell>(fields.get_cell(base_x+2,base_y).unwrap()).unwrap().is_alive = true;
}

fn set_pattern_glider(fields: &mut ResMut<Fields>,query:&mut  Query<&mut Cell>) {

    query.iter_mut().for_each(|mut cell| {
        cell.is_alive = false;
    });


    let base_x = (FIELD_WIDTH as f32/2.).floor() as isize;
    let base_y = (FIELD_HEIGHT as f32/2.).floor() as isize;

    let cells: [[isize;3];3] = [
        [1,1,1],
        [1,0,0],
        [0,1,0],
    ];
    for (y,row) in cells.into_iter().enumerate() {
        for (x, alive_bit) in row.into_iter().enumerate() {
            query.get_component_mut::<Cell>(fields.get_cell((x as isize)+base_x,(y as isize)+base_y).unwrap()).unwrap().is_alive = alive_bit == 1;
        }
    }
}


fn set_pattern_glider_gun(fields: &mut ResMut<Fields>,query:&mut  Query<&mut Cell>) {

    query.iter_mut().for_each(|mut cell| {
        cell.is_alive = false;
    });


    let base_x = (FIELD_WIDTH as f32/3.).floor() as isize;
    let base_y = (FIELD_HEIGHT as f32/3.).floor() as isize;

    let cells: [[isize;38];11] = [
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0],
        [0,1,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,1,1,0,0,0,0,0,0,0,0,1,0,0,0,1,0,1,1,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    for (y,row) in cells.into_iter().enumerate() {
        for (x, alive_bit) in row.into_iter().enumerate() {
            query.get_component_mut::<Cell>(fields.get_cell((x as isize)+base_x,(y as isize)+base_y).unwrap()).unwrap().is_alive = alive_bit == 1;
        }
    }
}


fn set_pattern_galaxy(fields: &mut ResMut<Fields>,query:&mut  Query<&mut Cell>) {

    query.iter_mut().for_each(|mut cell| {
        cell.is_alive = false;
    });

    let base_x = (FIELD_WIDTH as f32/2.5).floor() as isize;
    let base_y = (FIELD_HEIGHT as f32/2.5).floor() as isize;

    let cells: [[isize;15];15] = [
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,1,1,0,1,1,1,1,1,1,0,0,0,],
        [0,0,0,1,1,0,1,1,1,1,1,1,0,0,0,],
        [0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,1,1,0,0,0,0,0,1,1,0,0,0,],
        [0,0,0,1,1,0,0,0,0,0,1,1,0,0,0,],
        [0,0,0,1,1,0,0,0,0,0,1,1,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,],
        [0,0,0,1,1,1,1,1,1,0,1,1,0,0,0,],
        [0,0,0,1,1,1,1,1,1,0,1,1,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    ];
    for (y,row) in cells.into_iter().enumerate() {
        for (x, alive_bit) in row.into_iter().enumerate() {
            query.get_component_mut::<Cell>(fields.get_cell((x as isize)+base_x,(y as isize)+base_y).unwrap()).unwrap().is_alive = alive_bit == 1;
        }
    }
}


fn set_pattern_52513m(fields: &mut ResMut<Fields>,query:&mut  Query<&mut Cell>) {

    query.iter_mut().for_each(|mut cell| {
        cell.is_alive = false;
    });

    let base_x = (FIELD_WIDTH as f32/2.5).floor() as isize;
    let base_y = (FIELD_HEIGHT as f32/2.5).floor() as isize;

    let cells: [[isize;16];16] = [
        [1,1,1,0,0,1,1,0,1,0,1,1,0,1,1,1],
        [1,1,0,1,0,1,1,1,0,0,0,0,1,0,1,0],
        [0,1,0,0,1,0,0,1,0,1,0,1,1,1,0,1],
        [0,0,1,0,0,1,1,0,0,0,1,0,0,1,0,0],
        [0,0,1,0,0,0,0,0,1,0,1,0,0,0,1,1],
        [1,0,0,0,0,1,1,0,0,0,1,1,1,0,1,0],
        [0,0,0,1,1,0,0,1,0,0,1,0,1,0,0,1],
        [0,0,1,1,1,1,0,1,0,0,1,0,1,1,0,0],
        [1,1,0,1,1,0,0,1,1,0,0,0,0,0,1,1],
        [1,0,1,1,1,1,0,1,0,0,0,0,1,1,1,0],
        [1,0,0,0,1,1,1,1,0,0,1,1,1,0,0,0],
        [0,1,1,1,1,1,1,1,1,1,1,0,0,1,1,1],
        [1,1,0,0,0,1,0,1,1,1,0,1,0,1,1,1],
        [0,1,1,0,1,1,1,1,1,1,0,0,0,1,0,1],
        [1,0,1,0,0,0,0,0,1,1,1,1,0,1,0,0],
        [1,1,1,0,1,0,1,0,1,1,0,0,0,0,0,1],
    ];
    for (y,row) in cells.into_iter().rev().enumerate() {
        for (x, alive_bit) in row.into_iter().enumerate() {
            query.get_component_mut::<Cell>(fields.get_cell((x as isize)+base_x,(y as isize)+base_y).unwrap()).unwrap().is_alive = alive_bit == 1;
        }
    }
}


fn set_pattern_random(fields: &mut ResMut<Fields>,query: &mut Query<&mut Cell>) {

    query.iter_mut().for_each(|mut cell| {
        cell.is_alive = false;
    });


    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            let mut cell = query.get_component_mut::<Cell>(fields.get_cell(x as isize, y as isize).unwrap()).unwrap();
            let is_alive = rand::random::<bool>();
            cell.is_alive = is_alive;
        }
    }
}



fn move_forward(mut timer: ResMut<WorldTimer>, time: Res<Time>, _commands: Commands, _gizmos: Gizmos,fields: ResMut<Fields>, mut query: Query<&mut Cell>, config: Res<Config>) {
    timer.0.tick(time.delta());

    if !timer.0.finished() {
        return;
    }
    timer.0 = Timer::from_seconds(config.interval_ms as f32 /1000., TimerMode::Once);


    let mut new_is_alived_list = VecDeque::new();

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {

            let _x = x as isize;
            let _y = y as isize;

            let cells = vec!(
                fields.get_cell(_x-1, _y-1),
                fields.get_cell(_x-1, _y),
                fields.get_cell(_x-1, _y+1),
                fields.get_cell(_x, _y-1),
                fields.get_cell(_x, _y+1),
                fields.get_cell(_x+1, _y-1),
                fields.get_cell(_x+1, _y),
                fields.get_cell(_x+1, _y+1)
            ).into_iter().filter(|entity| {
                if entity.is_none() {return false;}
                let cell = query.get_component_mut::<Cell>(entity.unwrap());
                cell.unwrap().is_alive
            }).collect::<Vec<_>>();

            let alive_cell_count = cells.len();

            if let Ok(cell) = query.get_component::<Cell>(fields.get_cell(_x, _y).unwrap()) {
                let is_alive = if cell.is_alive {
                    config.alive_rule.will_spawn(alive_cell_count as u8)
                } else {
                    config.dead_rule.will_spawn(alive_cell_count as u8)
                };

                new_is_alived_list.push_back(is_alive);
            }
        }
    }
    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            // pop_front() to Vec
            let is_alived = new_is_alived_list.pop_front().unwrap();
            // let is_alived = new_is_alived_list.pop_front();
            let mut cell = query.get_component_mut::<Cell>(fields.get_cell(x as isize, y as isize).unwrap()).unwrap();
            cell.is_alive = is_alived;
        }
    }

}

fn display_cells(mut commands: Commands, mut gizmos: Gizmos,_time: Res<Time>,fields: ResMut<Fields>, query: Query<&mut Cell>, _config: Res<Config>) {

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            gizmos.rect_2d(
                Vec2::new(fields.get_x(x) as f32, fields.get_y(y)as f32),
                0.,
                Vec2::splat(CELL_SIZE as f32),
                Color::BLACK,
            );

            let entity = fields.get_cell(x as isize, y as isize).unwrap();
            if let Ok(cell) = query.get_component::<Cell>(entity) {
                if cell.is_alive {
                    commands.entity(entity).insert(Visibility::Visible);
                } else {
                    commands.entity(entity).insert(Visibility::Hidden);
                }
            }
        }
    }

    
}


fn ui_example_system(mut contexts: EguiContexts, mut config: ResMut<Config>, mut fields: ResMut<Fields>, mut query: Query<&mut Cell>) {

    egui::Window::new("Conway's Game of Life").show(contexts.ctx_mut(), |ui| {

        ui.add(egui::Slider::new(&mut config.interval_ms, 10..=5000).text("速度"));

        ui.horizontal(|ui| {
            ui.label("rule when alive");
            ui.add(egui::Slider::new(&mut config.alive_rule.min, 0..=8).text("最小セル数"));
            ui.add(egui::Slider::new(&mut config.alive_rule.max, 0..=8).text("最大セル数"));
        });
        ui.horizontal(|ui| {
            ui.label("rule when dead");
            ui.add(egui::Slider::new(&mut config.dead_rule.min, 0..=8).text("最小セル数"));
            ui.add(egui::Slider::new(&mut config.dead_rule.max, 0..=8).text("最大セル数"));
        });


        if ui.button("ランダム").clicked() {
            set_pattern_random(&mut fields,&mut query);
        }
        if ui.button("Blinkerパターン").clicked() {
            set_pattern_blinker(&mut fields, &mut query);
        }
        if ui.button("Gliderパターン").clicked() {
            set_pattern_glider(&mut fields, &mut query);
        }
        if ui.button("Glider Gunパターン").clicked() {
            set_pattern_glider_gun(&mut fields, &mut query);
        }
        if ui.button("Galaxyパターン").clicked() {
            set_pattern_galaxy(&mut fields, &mut query);
        }
        if ui.button("52513メトセトラパターン").clicked() {
            set_pattern_52513m(&mut fields, &mut query);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_deque() {
        let mut v = VecDeque::new();
        v.push_back(1);
        v.push_back(2);

        assert_eq!(1, v.pop_front().unwrap());
    }
}