use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct Color(pub Vec3);

#[derive(Debug, Clone, Copy)]
pub struct Point3(pub Vec3);

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub(self) e: [f64; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0., 0., 0.] }
    }
}

impl Vec3 {
    pub const fn zero() -> Self {
        Self { e: [0., 0., 0.] }
    }
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }
    pub fn to_color(&self) -> (usize, usize, usize) {
        let x = (self.e[0] * 255.999) as usize;
        let y = (self.e[1] * 255.999) as usize;
        let z = (self.e[2] * 255.999) as usize;
        (x, y, z)
    }
    pub fn to_unit(&self) -> Self {
        let len = self.len();
        Self {
            e: [self.e[0] / len, self.e[1] / len, self.e[2] / len],
        }
    }
    pub fn get_x(&self) -> f64 {
        self.e[0]
    }
    pub fn get_y(&self) -> f64 {
        self.e[1]
    }
    pub fn get_z(&self) -> f64 {
        self.e[2]
    }
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn dot_mul(&self, rhs: Self) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }
    pub fn cross_mul(&self, rhs: Self) -> Self {
        Self {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }
    pub const fn to_tuple(&self) -> (f64, f64, f64) {
        (self.e[0], self.e[1], self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}
impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self)
    }
}
//Dot Multiply
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

// 以下是语法糖
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])?;
        Ok(())
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0.e[0], self.0.e[1], self.0.e[2])
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0.e[0], self.0.e[1], self.0.e[2])
    }
}

impl Color {
    pub const fn white() -> Self {
        Color(Vec3::new(1., 1., 1.))
    }
    pub const fn red() -> Self {
        Color(Vec3::new(1., 0., 0.))
    }
    pub const fn green() -> Self {
        Color(Vec3::new(0., 1., 0.))
    }
    pub const fn blue() -> Self {
        Color(Vec3::new(0., 0., 1.))
    }
    pub const fn black() -> Self {
        Color(Vec3::new(0., 0., 0.))
    }
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Color(Vec3::new(x, y, z))
    }
    pub const fn to_tuple(&self) -> (f64, f64, f64) {
        (self.0.e[0], self.0.e[1], self.0.e[2])
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl Point3 {
    pub const fn zero() -> Self {
        Point3(Vec3::new(0., 0., 0.))
    }
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Point3(Vec3::new(x, y, z))
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Vec3::new(
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        )
    }
}
