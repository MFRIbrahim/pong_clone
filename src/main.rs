use bevy::{
    prelude::*,
    render::pass::ClearColor,
    window::WindowResizeConstraints,
};

use physics::*;
use components::*;

mod physics;
mod components;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "pong_clone".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resize_constraints: WindowResizeConstraints {
                min_width: WINDOW_WIDTH,
                min_height: WINDOW_HEIGHT,
                max_width: WINDOW_WIDTH,
                max_height: WINDOW_HEIGHT,
            },
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(Scoreboard {
            score_left: 0,
            score_right: 0, 
        })
        .add_startup_system(setup.system())
        .add_system(paddle_left_movement.system())
        .add_system(paddle_right_movement.system())
        .add_system(ball_movement.system())
        .add_system(scoreboard_counting.system())
        .add_system(collision.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Spawn the two Paddles
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 180.0)),
            ..Default::default()
        })
        .insert(PaddleLeft { speed: 1200.0 })
        .insert(Collider::Solid);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 180.0)),
            ..Default::default()
        })
        .insert(PaddleRight { speed: 1200.0 })
        .insert(Collider::Solid);

    // Spawn the ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(-1.0, -1.0, 0.0),
        });

    // Spawn the scoreboards
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(WINDOW_HEIGHT / 2.0 - 70.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Cousine-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText::Left);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(WINDOW_HEIGHT / 2.0 + 560.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Cousine-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText::Right);

    // Spawn the upper and lower wall from which the ball can bounce
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.04, 0.04, 0.04).into()),
            transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, 15.0)),
            ..Default::default()
        })
        .insert(Collider::Solid);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.04, 0.04, 0.04).into()),
            transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, 15.0)),
            ..Default::default()
        })
        .insert(Collider::Solid);

    // Spawn the left and right wall which increase the score when the ball touches them
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.04, 0.04, 0.04).into()),
            transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(15.0, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .insert(Collider::ScoreableLeft);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.04, 0.04, 0.04).into()),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(15.0, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .insert(Collider::ScoreableRight);

    // Spawn the central separator line
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(1.0, WINDOW_HEIGHT)),
        ..Default::default()
    });
}

fn scoreboard_counting(scoreboard: Res<Scoreboard>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, score_text) in query.iter_mut() {
        if let ScoreText::Left = *score_text {
            text.sections[0].value = format!("{}", scoreboard.score_left);
        }
        if let ScoreText::Right = *score_text {
            text.sections[0].value = format!("{}", scoreboard.score_right);
        }
    }
}
