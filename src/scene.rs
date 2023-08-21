use bevy::prelude::*;

use crate::*;

pub fn spawn_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("paddle.png");

    commands
        .spawn(SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: -20.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(Player {
            player_id: PlayerId::Left,
        });
    commands
        .spawn(SpriteBundle {
            texture: texture_handle,
            transform: Transform {
                translation: Vec3 {
                    x: 20.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(Player {
            player_id: PlayerId::Right,
        });

    let texture_handle = asset_server.load("ball.png");
    commands
        .spawn(SpriteBundle {
            texture: texture_handle.clone(),
            ..default()
        })
        .insert(Ball::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        mod startup {
            use super::*;

            #[test]
            fn scene_contains_left_and_right_players() {
                let mut app = App::new();
                app.add_plugins(bevy_asset::AssetPlugin::default());
                app.add_systems(Startup, spawn_scene);
                app.update();

                let player_count = app.world.query::<&Player>().iter(&app.world).len();
                assert_eq!(player_count, 2);
            }
    }
     */
}
