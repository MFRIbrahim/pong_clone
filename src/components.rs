use bevy::prelude::*;

pub struct PaddleLeft {
    pub speed: f32,
}

pub struct PaddleRight {
    pub speed: f32,
}

pub struct Ball {
    pub velocity: Vec3,
}

pub struct Scoreboard {
    pub score_left: usize,
    pub score_right: usize,
}

pub enum ScoreText {
    Left,
    Right,
}

pub enum Collider {
    Solid,
    ScoreableLeft,
    ScoreableRight,
}
