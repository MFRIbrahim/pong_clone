use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::seq::SliceRandom;

use crate::WINDOW_HEIGHT;
use crate::components::*;


pub fn paddle_left_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_left_query: Query<(&PaddleLeft, &mut Transform)>,
) {
    if let Ok((paddle_left, mut transform)) = paddle_left_query.single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::W) {
            direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= 1.0;
        }

        let translation = &mut transform.translation;
        translation.y += time.delta_seconds() * direction * paddle_left.speed;
        translation.y = translation
            .y
            .min(WINDOW_HEIGHT / 2.0 - 90.0)
            .max(-WINDOW_HEIGHT / 2.0 + 90.0);
    }
}

pub fn paddle_right_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_right_query: Query<(&PaddleRight, &mut Transform)>,
) {
    if let Ok((paddle_right, mut transform)) = paddle_right_query.single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= 1.0;
        }

        let translation = &mut transform.translation;
        translation.y += time.delta_seconds() * direction * paddle_right.speed;
        translation.y = translation
            .y
            .min(WINDOW_HEIGHT / 2.0 - 90.0)
            .max(-WINDOW_HEIGHT / 2.0 + 90.0);
    }
}

pub fn ball_movement(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation += ball.velocity * time.delta_seconds();
    }
}

pub fn collision(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(Entity, &mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(&Collider, &Transform, &Sprite)>,
) {
    if let Ok((ball_entity, mut ball, ball_transform, ball_sprite)) = ball_query.single_mut() {
        for (collider, collider_transform, collider_sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.size,
                collider_transform.translation,
                collider_sprite.size,
            );
            if let Some(collision) = collision {
                if let Collider::Solid = *collider {
                    let mut reflect_x = false;
                    let mut reflect_y = false;

                    match collision {
                        Collision::Left => reflect_x = ball.velocity.x > 0.0,
                        Collision::Right => reflect_x = ball.velocity.x < 0.0,
                        Collision::Top => reflect_y = ball.velocity.y < 0.0,
                        Collision::Bottom => reflect_y = ball.velocity.y > 0.0,
                    }

                    if reflect_x {
                        ball.velocity.x = -ball.velocity.x * 1.1;
                    }
                    if reflect_y {
                        ball.velocity.y = -ball.velocity.y * 1.1;
                    }
                }
                
                if let Collider::ScoreableLeft = *collider {
                    let ball_options: Vec::<f32> = vec![1.0, -1.0];
                    let choice_x = ball_options.choose(&mut rand::thread_rng()).unwrap();
                    let choice_y = ball_options.choose(&mut rand::thread_rng()).unwrap();
                    scoreboard.score_right += 1;
                    commands.entity(ball_entity).despawn();
                    commands
                        .spawn_bundle(SpriteBundle {
                            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
                            transform: Transform::from_xyz(0.0, 0.0, 2.0),
                            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                            ..Default::default()
                        })
                        .insert(Ball {
                            velocity: 400.0 * Vec3::new(*choice_x, *choice_y, 0.0),
                        });
                }
                if let Collider::ScoreableRight = *collider {
                    let ball_options: Vec::<f32> = vec![1.0, -1.0];
                    let choice_x = ball_options.choose(&mut rand::thread_rng()).unwrap();
                    let choice_y = ball_options.choose(&mut rand::thread_rng()).unwrap();
                    scoreboard.score_left += 1;
                    commands.entity(ball_entity).despawn();
                    commands
                        .spawn_bundle(SpriteBundle {
                            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
                            transform: Transform::from_xyz(0.0, 0.0, 2.0),
                            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                            ..Default::default()
                        })
                        .insert(Ball {
                            velocity: 400.0 * Vec3::new(*choice_x, *choice_y, 0.0),
                        });
                }
            }
        }
    }
}
