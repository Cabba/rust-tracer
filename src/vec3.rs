use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

/// Linear interpolation between two vectors, this functions implements
/// the following equation:
/// $$
///    start * (1 - t) + end * t
/// $$
pub fn lerp(start: &Vec3, end: &Vec3, t: f64) -> Vec3 {
    (*start) * (1.0 - t) + (*end) * t
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    data: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { data: [x, y, z] }
    }

    pub fn zero() -> Self {
        Vec3::new(0., 0., 0.)
    }

    pub fn unit() -> Self {
        Vec3::new(1., 1., 1.)
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn length2(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        return self.length2().sqrt();
    }

    /// Return a new normalized Vec3 with the same direction as the original Vec3
    pub fn normal(&self) -> Vec3 {
        return self.clone() / self.length();
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self[1] * v[2] - self[2] * v[1],
            self[0] * v[2] - self[2] * v[0],
            self[0] * v[1] - self[1] * v[0],
        )
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Self::Output {
        Self {
            data: [self.x() + v.x(), self.y() + v.y(), self.z() + v.z()],
        }
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Self::Output {
        Self::Output {
            data: [self + v.x(), self + v.y(), self + v.z()],
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self[0] += v[0];
        self[1] += v[1];
        self[2] += v[2];
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Self::Output {
        Self {
            data: [self.x() - v.x(), self.y() - v.y(), self.z() - v.z()],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Self {
            data: [self.x() * v.x(), self.y() * v.y(), self.z() * v.z()],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: f64) -> Self::Output {
        Self {
            data: [self.x() * v, self.y() * v, self.z() * v],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Self::Output {
            data: [self * v.x(), self * v.y(), self * v.z()],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, v: f64) {
        self[0] *= v;
        self[1] *= v;
        self[2] *= v;
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        self[0] *= v[0];
        self[1] *= v[1];
        self[2] *= v[2];
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, v: f64) -> Self::Output {
        Self {
            data: [self.x() / v, self.y() / v, self.z() / v],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, v: f64) {
        *self *= 1.0 / v;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            data: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

// Annotation used to tell rust compiler to compile this code only if running tests
#[cfg(test)]
mod vec3_tests {
    // import in the module all the stuff in the upper context
    use super::*;

    #[test]
    fn accessors() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);

        let z = Vec3::zero();
        assert_eq!(z.x(), 0.0);
        assert_eq!(z.y(), 0.0);
        assert_eq!(z.z(), 0.0);
    }

    #[test]
    fn access_index() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 2.0);
        assert_eq!(p[2], 3.0);
    }

    #[test]
    fn modify_index() {
        let mut p = Vec3::new(1., 2., 3.);
        assert_eq!(p[2], 3.0);
        p[2] = 4.;
        assert_eq!(p[2], 4.0);
    }
}
