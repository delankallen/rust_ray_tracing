use std::{ops::{self, Add, Mul, Neg, Range, Sub}};
use rand::{Rng, distributions::Standard, prelude::Distribution};
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 (pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }

    #[inline]
    pub fn zip_with(self, other: Vec3, mut f: impl FnMut(f32, f32) -> f32) -> Self {
        Vec3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2))
    }

    #[inline]
    pub fn zip_with3(
        self,
        other1: Vec3,
        other2: Vec3,
        mut f: impl FnMut(f32, f32, f32) -> f32,
    ) -> Self {
        Vec3(
            f(self.0, other1.0, other2.0),
            f(self.1, other1.1, other2.1),
            f(self.2, other1.2, other2.2),
        )
    }

    #[inline]
    pub fn reduce(self, f: impl Fn(f32, f32) -> f32) -> f32 {
        f(f(self.0, self.1), self.2)
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    #[inline]
    pub fn dot(&self, other:Self) -> f32 {
        self.zip_with(other, Mul::mul).reduce(Add::add)
    }

    pub fn cross(&self, other: Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            -(self.0 * other.2 - self.2 * other.0),
            self.0 * other.1 - self.1 * other.0
        )
    }

    #[inline]
    pub fn unit_vector (self) -> Self {
        self / self.length()
    }

    #[inline]
    pub fn map(self, mut f: impl FnMut(f32) -> f32) -> Self {
        Vec3(f(self.0), f(self.1), f(self.2))
    }

    pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Self {
        loop {
            let p = 2. * rng.gen::<Vec3>() - Vec3::from(1.);
            if p.length_squared() >= 1.0 { continue };
            return p;
        }
    }

    pub fn random_unit_vector(rng: &mut impl Rng) -> Self {
        return Vec3::random_in_unit_sphere(rng).unit_vector();
    }
    
    pub fn random_in_unit_disc(rng: &mut impl Rng) -> Self {
        loop {
            let p = Vec3(rng.gen(), rng.gen(), 0.) - Vec3(1.,1.,0.);
            if p.dot(p) < 1.0 { return p }
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(rng: &mut impl Rng, range:Range<f32>) -> Self {
        Vec3(rng.gen_range(range.clone()), rng.gen_range(range.clone()), rng.gen_range(range.clone()))
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

impl std::ops::Add<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| self + x)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, Add::add)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, Sub::sub)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, Mul::mul)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self) * rhs
    }
}

impl std::ops::Div for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Div::div)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

impl From<f32> for Vec3 {
    #[inline]
    fn from(v: f32) -> Self {
        Vec3(v, v, v)
    }
}

impl std::iter::Sum for Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::default(), std::ops::Add::add)
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Distribution<Vec3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

#[inline]
pub fn reflect(v:Vec3, n:Vec3) -> Vec3 {
    v - 2.* v.dot(n) * n
}

#[inline]
pub fn refract(uv:Vec3, n:Vec3, etai_over_etat:f32) -> Vec3 {
    let cos_theta = f32::min((-uv).dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt().neg() * n;
    r_out_perp + r_out_parallel
}