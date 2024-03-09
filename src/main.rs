use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GenTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
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
    for i in 0..12 {
        spawn_cell(&mut commands, &mut meshes, &mut materials, i);
    }
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
    position: u32,
) {
    let alive = rand::random();
    commands.spawn(CellBundle {
        life: Life(alive),
        sprite: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0))),
            material: materials.add(life_to_color(alive)),
            transform: Transform::from_xyz(50. * position as f32, 0.0, 0.0),
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

fn life_to_color(life: bool) -> Color {
    if life {
        Color::hsl(72., 0.99, 0.5)
    } else {
        Color::hsl(252., 0.99, 0.5)
    }
}