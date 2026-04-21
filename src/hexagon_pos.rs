use std::{f32::consts::PI, ops::Add};

use bevy::math::Vec2;
use serde::Serialize;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize)]
pub struct HexagonPos {
    pub x: i32,
    pub y: i32,
}

impl HexagonPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn real_scaled(self, width: f32, height: f32, side: f32) -> Vec2 {
        let x = self.x as f32 * (width - side / 2.0);
        let y = (self.y * 2 + self.x.rem_euclid(2)) as f32 * height / 2.0;

        Vec2::new(x, y)
    }

    pub fn real(self) -> Vec2 {
        let (sin, cos) = (PI / 6.0).sin_cos();

        let x = self.x as f32 * (1.0 + sin);
        let y = (self.y * 2 - self.x % 2) as f32 * (cos);

        Vec2::new(x, y)
    }

    pub fn points(self, width: f32, height: f32, side: f32) -> [Vec2; 6] {
        let pos = self.real_scaled(width, height, side);

        [
            Vec2::new((width - side) / 2.0, 0.0),
            Vec2::new((width + side) / 2.0, 0.0),
            Vec2::new(width, height / 2.0),
            Vec2::new((width + side) / 2.0, height),
            Vec2::new((width - side) / 2.0, height),
            Vec2::new(0.0, height / 2.0),
        ]
        .map(|v| pos + v)
    }

    pub fn neighbours(self) -> [HexagonPos; 6] {
        if self.x.rem_euclid(2) == 0 {
            [
                HexagonPos::new(0, -1),
                HexagonPos::new(1, -1),
                HexagonPos::new(1, 0),
                HexagonPos::new(0, 1),
                HexagonPos::new(-1, 0),
                HexagonPos::new(-1, -1),
            ]
            .map(|h| self + h)
        } else {
            [
                HexagonPos::new(0, -1),
                HexagonPos::new(1, 0),
                HexagonPos::new(1, 1),
                HexagonPos::new(0, 1),
                HexagonPos::new(-1, 1),
                HexagonPos::new(-1, 0),
            ]
            .map(|h| self + h)
        }
    }
}

impl Add for HexagonPos {
    type Output = HexagonPos;

    fn add(self, rhs: Self) -> Self::Output {
        HexagonPos::new(self.x + rhs.x, self.y + rhs.y)
    }
}
