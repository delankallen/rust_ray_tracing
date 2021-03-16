use std::ops::{self, Range};
use rand::Rng;
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Vec3 {
    pub fn new(x: f64, y:f64, z:f64) -> Vec3 {
        Vec3 { e : [x,y,z]}
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(u:Self, v:Self) -> f64 {
        u.e[0]*v.e[0] + u.e[1]*v.e[1] + u.e[2]*v.e[2]
    }

    pub fn cross( u: Self, v: Self) -> Self {
        Self {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0]
            ]
        }
    }

    pub fn unit_vector (v:Self) -> Self {
        v/v.length()
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            e: [
                rng.gen(), rng.gen(), rng.gen()
            ]
        }
    }

    pub fn random_range(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            e: [
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone())
            ]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0]+other.e[0],
                self.e[1]+other.e[1],
                self.e[2]+other.e[2],
            ]
        };
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2],
            ]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [
                self.e[0]*rhs,
                self.e[1]*rhs,
                self.e[2]*rhs,
            ]
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0]+rhs.e[0],
                self.e[1]+rhs.e[1],
                self.e[2]+rhs.e[2],
            ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0]-rhs.e[0],
                self.e[1]-rhs.e[1],
                self.e[2]-rhs.e[2],
            ]
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0]*rhs.e[0],
                self.e[1]*rhs.e[1],
                self.e[2]*rhs.e[2],
            ]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [
                self.e[0]*rhs,
                self.e[1]*rhs,
                self.e[2]*rhs,
            ]
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0/rhs) * self
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0..1.0);
        if p.length_squared() >= 1.0 { continue };
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    Vec3::unit_vector(random_in_unit_sphere())
}