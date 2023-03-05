use rand::*;
use std::{
    fmt::Display,
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
    },
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
    pub fn random(rg: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            e: [
                rng.gen_range(rg.clone()),
                rng.gen_range(rg.clone()),
                rng.gen_range(rg),
            ],
        }
    }
    pub fn random_in_sphere(r: f64) -> Self {
        loop {
            let tmp = Self::random(-r..r);
            if tmp.len_squared() >= r * r {
                continue;
            } else {
                return tmp;
            }
        }
    }
    pub fn random_unit(r: f64) -> Self {
        Self::random_in_sphere(r).to_unit()
    }
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.e.iter().all(|&x| x.abs() < EPS)
    }
    pub fn reflect(&self, normal: Vec3) -> Self {
        *self - 2. * self.dot_mul(normal) * normal
    }
    pub fn refract(
        &self,
        normal: Vec3,
        etai_over_etat: f64, /*折射率之比，入:出 */
    ) -> Vec3 {
        let cos_theta_1 = -self.dot_mul(normal).min(1.0);
        let r_out_perp/*垂直分量 */ = etai_over_etat * (*self + cos_theta_1*normal);
        let r_out_parallel/*平行分量 */ = -(1.0- r_out_perp.len_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
                0.,
            );
            if p.len_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
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

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self)
    }
}

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

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs)
    }
}
impl Mul<Color> for f64 {
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
        Color(Vec3::new(
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ))
    }
}
impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color(Vec3::zero()), |a, b| {
            Color(Vec3::new(a.0[0] + b.0[0], a.0[1] + b.0[1], a.0[2] + b.0[2]))
        })
    }
}
impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(Vec3::new(
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ))
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
impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point3(Vec3::new(
            self.0[0] + rhs[0],
            self.0[1] + rhs[1],
            self.0[2] + rhs[2],
        ))
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
