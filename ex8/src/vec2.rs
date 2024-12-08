use std::ops;

#[derive(Eq, Hash, PartialEq)]
pub struct Vec2 {
    pub a: i32,
    pub b: i32,
}

impl ops::Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Self::Output {
        Vec2{
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl ops::Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, other: &Vec2) -> Self::Output {
        Vec2{
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

impl ops::Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: i32) -> Self::Output {
        Vec2{
            a: self.a * other,
            b: self.b * other,
        }
    }
}

impl Vec2 {
    pub fn is_inside(&self, boundary: &Vec2) -> bool {
        if self.a.is_negative() || self.b.is_negative() {
            return false;
        }
        if self.a < boundary.a && self.b < boundary.b {
            return true;
        }
        false
    }
}
