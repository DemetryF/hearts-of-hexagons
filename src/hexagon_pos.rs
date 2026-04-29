use {
    bevy::math::Vec2,
    serde::Serialize,
    std::{f32::consts::PI, ops::Add},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize)]
pub struct HexagonPos {
    pub x: i32,
    pub y: i32,
}

impl HexagonPos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn real_regular(self, side: f32) -> Vec2 {
        let cos = (PI / 6.0).cos();

        return self.real_scaled(2. * side, side * cos * 2., side);
    }

    pub fn real_scaled(self, width: f32, height: f32, side: f32) -> Vec2 {
        let x = self.x as f32 * (width / 2. + side / 2.0);
        let y = (self.y * 2 - self.x % 2/*self.x.rem_euclid(2)*/) as f32 * height / 2.0;

        Vec2::new(x, y)
    }

    pub fn from_real_regular(p: Vec2, side: f32) -> Self {
        let cos = (PI / 6.0).cos();

        return Self::from_real_scaled(p, 2. * side, side * cos * 2., side);
    }

    pub fn from_real_scaled(p: Vec2, width: f32, height: f32, side: f32) -> Self {
        let x_step = width / 2.0 + side / 2.0;

        let approx_x = (p.x / x_step).round() as i32;

        let mut best = HexagonPos::new(0, 0);
        let mut best_dist = f32::INFINITY;

        for hx in approx_x - 1..=approx_x + 1 {
            let parity = hx.rem_euclid(2) as f32;
            let approx_y = ((p.y * 2.0 / height + parity) / 2.0).round() as i32;

            for hy in approx_y - 1..=approx_y + 1 {
                let pos = HexagonPos::new(hx, hy);
                let d = pos.real_scaled(width, height, side).distance_squared(p);
                if d < best_dist {
                    best_dist = d;
                    best = pos;
                }
            }
        }

        best
    }

    pub fn sides_regular(self, side: f32) -> [(Vec2, Vec2); 6] {
        let cos = (PI / 6.0).cos();

        self.sides(2. * side, side * cos * 2., side)
    }

    pub fn sides(self, width: f32, height: f32, side: f32) -> [(Vec2, Vec2); 6] {
        let p = self.points(width, height, side);
        [
            (p[0], p[1]), // NE
            (p[1], p[2]), // N
            (p[2], p[3]), // NW
            (p[3], p[4]), // SW
            (p[4], p[5]), // S
            (p[5], p[0]), // SE
        ]
    }

    pub fn neighbours(self) -> [HexagonPos; 6] {
        let HexagonPos { x, y } = self;

        let shift = if x.rem_euclid(2) == 0 { 0 } else { -1 };

        [
            HexagonPos::new(x + 1, y + 1 + shift), // NE
            HexagonPos::new(x, y + 1),             // N
            HexagonPos::new(x - 1, y + 1 + shift), // NW
            HexagonPos::new(x - 1, y + shift),     // SW
            HexagonPos::new(x, y - 1),             // S
            HexagonPos::new(x + 1, y + shift),     // SE
        ]
    }

    pub fn points(self, width: f32, height: f32, side: f32) -> [Vec2; 6] {
        let c = self.real_scaled(width, height, side);

        [
            c + Vec2::new(side, 0.0),
            c + Vec2::new(side / 2.0, height / 2.0),
            c + Vec2::new(-side / 2.0, height / 2.0),
            c + Vec2::new(-side, 0.0),
            c + Vec2::new(-side / 2.0, -height / 2.0),
            c + Vec2::new(side / 2.0, -height / 2.0),
        ]
    }

    pub fn manhattan_dist(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for HexagonPos {
    type Output = HexagonPos;

    fn add(self, rhs: Self) -> Self::Output {
        HexagonPos::new(self.x + rhs.x, self.y + rhs.y)
    }
}
