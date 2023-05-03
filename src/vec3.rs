use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Index, IndexMut, Mul, Range, Sub},
};

pub use glam::Vec3A as Vec3;
pub use glam::vec3a as vec3; 
use rand::*;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub Vec3);

#[derive(Debug, Clone, Copy)]
pub struct Point3(pub Vec3);
pub trait Vec3Funcs {
    //fn to_color(&self) -> (usize, usize, usize);

    fn refract(
        &self,
        normal: Vec3,
        etai_over_etat: f32, /*折射率之比，入:出 */
    ) -> Vec3;
    fn reflect(&self, normal: Vec3) -> Self;
    fn near_zero(&self) -> bool;
    fn random(rg: Range<f32>) -> Self;
    fn random_unit(r: f32) -> Self;
    fn random_in_sphere(r: f32) -> Self;
    fn random_in_unit_disk() -> Self;
}
/* 
impl From<Vec3> for Color{
    fn from(value: Vec3) -> Self {
        Color(value.to_color())
    }
}
*/

impl Vec3Funcs for Vec3 {
	/* 
    fn to_color(&self) -> (usize, usize, usize) {
        let x = (self[0] * 255.999) as usize;
        let y = (self[1] * 255.999) as usize;
        let z = (self[2] * 255.999) as usize;
        (x, y, z)
    }
	*/
    fn random(rg: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(rg.clone()),
            rng.gen_range(rg.clone()),
            rng.gen_range(rg),
        )
    }
    fn random_in_sphere(r: f32) -> Self {
        loop {
            let tmp = Self::random(-r..r);
            if tmp.length_squared() >= r * r {
                continue;
            } else {
                return tmp;
            }
        }
    }
    fn random_unit(r: f32) -> Self {
        Self::random_in_sphere(r).normalize()
    }
    fn near_zero(&self) -> bool {
        const EPS: f32 = 1e-6;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
    fn reflect(&self, normal: Vec3) -> Self {
        *self - 2. * self.dot(normal) * normal
    }
    fn refract(
        &self,
        normal: Vec3,
        etai_over_etat: f32, /*折射率之比，入:出 */
    ) -> Vec3 {
        let cos_theta_1 = -self.dot(normal).min(1.0);
        let r_out_perp/*垂直分量 */ = etai_over_etat * (*self + cos_theta_1*normal);
        let r_out_parallel/*平行分量 */ = -(1.0- r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
    fn random_in_unit_disk() -> Self {
        loop {
            let p = vec3(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
                0.,
            );
            if p.length_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

impl Color {
	pub const WHITE: Self = Self(vec3(1., 1., 1.));
    pub const RED: Self = Self(vec3(1., 0., 0.));
    pub const GREEN: Self = Self(vec3(0., 1., 0.));
	pub const BLUE: Self = Self(vec3(0., 0., 1.));
    pub const BLACK: Self = Self(vec3(0., 0., 0.));
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Color(vec3(x, y, z))
    }
    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.0[0], self.0[1], self.0[2])
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color(self.0 * rhs)
    }
}
impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0)
    }
}
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}
impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color(vec3(
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ))
    }
}
impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color(Vec3::ZERO), |a, b| {
            Color(vec3(a.0[0] + b.0[0], a.0[1] + b.0[1], a.0[2] + b.0[2]))
        })
    }
}
impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(vec3(
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ))
    }
}

impl Point3 {
    pub const fn zero() -> Self {
        Point3(vec3(0., 0., 0.))
    }
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Point3(vec3(x, y, z))
    }
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    pub fn y(&self) -> f32 {
        self.0[1]
    }
    pub fn z(&self) -> f32 {
        self.0[2]
    }
}
impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point3(vec3(
            self.0[0] + rhs[0],
            self.0[1] + rhs[1],
            self.0[2] + rhs[2],
        ))
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Point3::new(self.0[0] - rhs[0], self.0[1] - rhs[1], self.0[2] - rhs[2])
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Point3) -> Self::Output {
        vec3(self.0[0] - rhs[0], self.0[1] - rhs[1], self.0[2] - rhs[2])
    }
}

// 以下是语法糖
impl Index<usize> for Point3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Point3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul<f32> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f32) -> Self::Output {
        Point3::new(self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs)
    }
}
