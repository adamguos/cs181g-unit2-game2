#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct RotatedRect {
    // x and y are the coords of the center
    // w and h are width and height from the AngledRect's angle
    pub x: f64,
    pub y: f64,
    pub w: u16,
    pub h: u16,
    pub rotation: f64,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vec2i(pub i32, pub i32);

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Vec2f(pub f64, pub f64);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rect {
    pub fn _contains(&self, point: &Vec2i) -> bool {
        point.0 >= self.x
            && point.0 <= self.x + self.w as i32
            && point.1 >= self.y
            && point.1 <= self.y + self.h as i32
    }

    pub fn contains_f(&self, point: &Vec2f) -> bool {
        point.0 >= self.x as f64
            && point.0 <= self.x as f64 + self.w as f64
            && point.1 >= self.y as f64
            && point.1 <= self.y as f64 + self.h as f64
    }
}

// Feel free to add impl blocks with convenience functions
impl RotatedRect {
    pub fn corners(&self) -> Vec<Vec2f> {
        // Vectors from center to sides, perpendicular to sides
        let perp1 = (
            self.rotation.cos() * self.w as f64 / 2.0,
            self.rotation.sin() * self.w as f64 / 2.0,
        );
        let perp2 = (
            -self.rotation.sin() * self.h as f64 / 2.0,
            self.rotation.cos() * self.h as f64 / 2.0,
        );

        let mut cs: Vec<Vec2f> = vec![];
        cs.push(Vec2f(
            self.x + perp1.0 + perp2.0,
            self.y + perp1.1 + perp2.1,
        ));
        cs.push(Vec2f(
            self.x + perp1.0 - perp2.0,
            self.y + perp1.1 - perp2.1,
        ));
        cs.push(Vec2f(
            self.x - perp1.0 - perp2.0,
            self.y - perp1.1 - perp2.1,
        ));
        cs.push(Vec2f(
            self.x - perp1.0 + perp2.0,
            self.y - perp1.1 + perp2.1,
        ));
        cs
    }
}

impl Vec2f {
    pub fn dot(&self, other: &Vec2f) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn norm(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn scalar_mult(&self, scalar: f64) -> Vec2f {
        Vec2f(self.0 * scalar, self.1 * scalar)
    }
}
