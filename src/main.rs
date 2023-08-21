mod scene;

use std::f32::consts::PI;

use bevy::prelude::*;

pub enum PlayerId {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub player_id: PlayerId,
}

#[derive(Component)]
pub struct Ball {
    pub direction: Vec2,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            direction: INITIAL_BALL_SPEED * Vec2::from_angle(PI / 4.0),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, scene::spawn_scene)
        .add_systems(Update, move_paddles)
        .add_systems(Update, move_ball)
        .run();
}

const PADDLE_SPEED: f32 = 100.0;
const INITIAL_BALL_SPEED: f32 = 100.0;

fn move_paddles(
    mut query: Query<(&mut Transform, &Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut query {
        let keys = match player.player_id {
            PlayerId::Left => (KeyCode::Q, KeyCode::A),
            PlayerId::Right => (KeyCode::Up, KeyCode::Down),
        };
        if keyboard_input.pressed(keys.0) {
            transform.translation.y += time.delta_seconds() * PADDLE_SPEED;
        }
        if keyboard_input.pressed(keys.1) {
            transform.translation.y -= time.delta_seconds() * PADDLE_SPEED;
        }
    }
}

fn move_ball(mut query: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    let (mut transform, ball) = query.single_mut();

    let movement = time.delta_seconds() * ball.direction;

    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
}
