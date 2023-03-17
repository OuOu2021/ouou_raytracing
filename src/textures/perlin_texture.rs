use std::array::from_fn;

use rand::{seq::SliceRandom, thread_rng};

use crate::vec3::Vec3;

use super::*;
static POINT_COUNT: usize = 256;

struct Perlin {
    // 足够均匀的三组0~255随机排列
    perm: [Vec<usize>; 3],
    // 足够均匀的随机向量
    rand_vec: Vec<Vec3>,
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            perm: [
                perlin_generate_perm(),
                perlin_generate_perm(),
                perlin_generate_perm(),
            ],
            rand_vec: (0..POINT_COUNT).map(|_| Vec3::random(-1.0..1.0)).collect(),
        }
    }
}

impl Perlin {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        // 晶格内位置
        let (u, v, w) = (
            p.x() - p.x().floor(),
            p.y() - p.y().floor(),
            p.z() - p.z().floor(),
        );
        // 处在哪一个晶格
        let floor = |x: f64| x.floor() as i32;
        let (i, j, k) = (floor(p.x()), floor(p.y()), floor(p.z()));

        // 获取周围八个晶格点，哈希到随机向量范围之内
        let mask = |x: i32, i: usize| self.perm[i][(x & 255) as usize];
        let c: [[[Vec3; 2]; 2]; 2] = from_fn(|di| {
            from_fn(|dj| {
                from_fn(|dk| {
                    self.rand_vec
                        [mask(i + di as i32, 0) ^ mask(j + dj as i32, 1) ^ mask(k + dk as i32, 2)]
                })
            })
        });
        perlin_interpolation(c, u, v, w)
    }
    /// 搅动
    pub fn turb_with_depth(&self, p: &Point3, depth: usize) -> f64 {
        let mut tmp_p = *p * 0.5;
        let mut weight = 2.;
        (0..depth)
            .map(|_| {
                weight *= 0.5;
                tmp_p = tmp_p * 2.;
                weight * self.noise(&tmp_p)
            })
            .sum::<f64>()
            .abs()
    }
    pub fn turb(&self, p: &Point3) -> f64 {
        self.turb_with_depth(p, 7)
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    permute((0..POINT_COUNT).collect::<Vec<usize>>())
}

fn permute(mut v: Vec<usize>) -> Vec<usize> {
    let mut rng = thread_rng();
    v.shuffle(&mut rng);
    v
}

fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    // Hermite插值
    let f = |x| x * x * (3. - 2. * x);
    let (uu, vv, ww) = (f(u), f(v), f(w));

    // ?
    let f2 = |x, y| x as f64 * y + (1. - x as f64) * (1. - y);

    // 遍历三维中相邻的八个晶格点
    (0..2)
        .map(|i| {
            (0..2)
                .map(|j| {
                    (0..2)
                        .map(|k| {
                            // 计算相对位置向量
                            let weight_vec = Vec3::new(u - i as f64, v - j as f64, w - k as f64);

                            // 梯度加权求和得出灰度
                            f2(i, uu) * f2(j, vv) * f2(k, ww) * c[i][j][k].dot_mul(weight_vec)
                        })
                        .sum::<f64>()
                })
                .sum::<f64>()
        })
        .sum()
}

/// 噪声纹理，目前只实现了柏林噪声
/// 之后可以把`Noise`抽象成`Trait`并添加更多随机纹理
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Default::default(),
            scale,
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self {
            noise: Perlin::new(),
            scale: 4.0,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: (f64, f64), p: Point3) -> Color {
        // 0.5*(1+x) 把-1~1的x转换到0~1之间，防止GAMMA的sqrt出现NaN
        Color::new(1., 1., 1.) * 0.5 * (1. + (p.z() * self.scale + 10. * self.noise.turb(&p)).sin())
    }
}
