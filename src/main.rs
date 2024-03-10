use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 20;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GenTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .add_systems(Startup, fit_canvas_to_parent)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let cells = GRID_WIDTH * GRID_HEIGHT;
    for i in 0..cells {
        spawn_cell(&mut commands, &mut meshes, &mut materials, i);
    }
}

fn fit_canvas_to_parent() {
    let canvas: HtmlCanvasElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("canvas")
        .unwrap()
        .unwrap()
        .unchecked_into();
    let style = canvas.style();
    style.set_property("width", "100%").unwrap();
    style.set_property("height", "100%").unwrap();
}

#[derive(Component)]
struct Life(bool);

#[derive(Bundle)]
struct CellBundle {
    life: Life,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Resource)]
struct GenTimer(Timer);

fn spawn_cell(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    index: u32,
) {
    let alive = rand::random();
    commands.spawn(CellBundle {
        life: Life(alive),
        sprite: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0))),
            material: materials.add(life_to_color(alive)),
            transform: index_to_transform(index),
            ..default()
        },
    });
}

fn toggle(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GenTimer>,
    mut query: Query<(Entity, &mut Life)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (e, mut life) in &mut query {
            let alive = !life.0;
            life.0 = alive;
            commands
                .entity(e)
                .insert(materials.add(life_to_color(alive)));
        }
    }
}

fn index_to_transform(index: u32) -> Transform {
    let cell_size = 50.;
    let y_offset = -cell_size * GRID_HEIGHT as f32 / 2.;
    let x_offset = -cell_size * GRID_WIDTH as f32 / 2.;
    let row = index / GRID_WIDTH;
    let column = index - row * GRID_WIDTH;
    let x = column as f32 * cell_size + x_offset;
    let y = row as f32 * cell_size + y_offset;
    Transform::from_xyz(x, y, 0.0)
}

fn life_to_color(life: bool) -> Color {
    if life {
        Color::hsl(72., 0.99, 0.5)
    } else {
        Color::hsl(252., 0.99, 0.5)
    }
}
