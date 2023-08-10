#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Point(pub i64, pub i64);

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Point {
    pub fn manhattan_distance(&self, other: Point) -> i64 {
        (other.0 - self.0).abs() + (other.1 - self.1).abs()
    }
}
