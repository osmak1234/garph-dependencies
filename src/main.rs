use std::process::{Command, Stdio};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let output = Command::new("cargo")
        .arg("tree")
        .arg("--prefix=depth")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut vertical_height = vec![0];

    for (i, line) in stdout.lines().enumerate() {
        let mut found_char = false;

        let depth = line
            .chars()
            .filter(|ch| {
                if ch.is_numeric() && !found_char {
                    true
                } else if !ch.is_numeric() && !found_char {
                    found_char = true;
                    false
                } else {
                    false
                }
            })
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(1);

        let y_cor = match vertical_height.get_mut(depth as usize) {
            Some(val) => {
                *val += 1;
                *val - 1
            }
            None => {
                vertical_height.push(1);
                0
            }
        };

        String::from( "sdfasdf" )

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(
                -150. + depth as f32 * 20.,
                -400. + y_cor as f32 * 6.,
                0.,
            )),
            ..default()
        });
    }
}
