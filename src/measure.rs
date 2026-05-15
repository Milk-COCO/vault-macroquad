use crate::get_context;
use crate::thread_assert;
use miniquad::window::screen_size;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub fn get_measure_ratio() -> Option<f64> {
    let context = get_context();
    context.measure_ratio
}

pub fn set_measure_ratio(r: Option<f64>) {
    let context = get_context();
    context.measure_ratio = r;
}

static mut DYN_POS: Option<(f32,f32)> = None;

pub fn set_dyn_pos(factor: impl Into<(f32,f32)>) {
    thread_assert::same_thread();
    #[cfg(not(target_os = "android"))]
    unsafe {
        DYN_POS = Some(factor.into());
    }
}

pub fn remove_dyn_pos() {
    thread_assert::same_thread();
    unsafe {
        DYN_POS = None;
    }
}

pub fn dyn_pos() -> Option<(f32,f32)> {
    thread_assert::same_thread();
    unsafe {
        DYN_POS.clone()
    }
}

pub trait MeasureVec: ToPhysicalVec + FromPhysicalVec
where Self: Sized
{
    fn new(x: f64, y: f64) -> Self;
    
    /// 零点
    #[inline]
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    
    /// 从单值创建（x=y）
    #[inline]
    fn splat(v: f64) -> Self {
        Self::new(v, v)
    }
    
    fn x(&self) -> f64;
    
    fn y(&self) -> f64;
    
    /// 实际像素大小之和
    #[inline]
    fn to_sum(&self) -> f32 {
        let phy = self.to_physical_vec();
        phy.0 + phy.1
    }
    
    fn comb(a: &Self,b: &Self) -> (f32, f32){
        (a.to_sum(), b.to_sum())
    }
    
    fn splat_comb(a: &Self) -> (f32, f32) {
        (a.to_sum(), a.to_sum())
    }
    
    fn hh(v: f64) -> (f32,f32) {
        let h = Self::h(v);
        (h,h)
    }
    
    fn ww(v: f64) -> (f32,f32) {
        let w = Self::w(v);
        (w,w)
    }
    
    fn w(w: f64) -> f32 {
        Self::new(w,0.0).to_sum()
    }
    
    fn h(h: f64) -> f32 {
        Self::new(0.0,h).to_sum()
    }
    
    fn wh(w: f64, h: f64) -> f32 {
        Self::new(w,h).to_sum()
    }
    
    /// VeC(v, v).to_sum()
    #[inline]
    fn whs(v: f64) -> f32 {
        Self::new(v, v).to_sum()
    }
    
    /// 转换为macroquad的Vec2（物理坐标）
    #[inline]
    fn to_mq_vec2(&self) -> crate::math::Vec2 {
        let (x, y) = self.to_physical_vec();
        crate::math::Vec2::new(x, y)
    }
    
    // ===== 几何常用方法 =====
    /// 取绝对值
    #[inline]
    fn abs(&self) -> Self {
        Self::new(self.x().abs(), self.x().abs())
    }
    
    /// 向量长度（相对坐标的欧几里得距离）
    #[inline]
    fn length(&self) -> f64 {
        (self.x().powi(2) + self.x().powi(2)).sqrt()
    }
    
    /// 向量长度的平方（避免开方，性能更高）
    #[inline]
    fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.x().powi(2)
    }
    
    /// 归一化（单位向量）
    #[inline]
    fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            Self::new(self.x() / len, self.x() / len)
        }
    }
    
    /// 与另一个坐标的距离（相对坐标）
    #[inline]
    fn distance(&self, other: &Self) -> f64 {
        Self::new(self.x() - other.x(), self.y() - other.y() ).length()
    }
    
    /// 点积
    #[inline]
    fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.x() * other.y()
    }
    
    /// 二维叉积（标量结果）
    #[inline]
    fn cross(&self, other: &Self) -> f64 {
        self.x() * other.y() - self.x() * other.x()
    }
    
    /// 线性插值
    #[inline]
    fn lerp(&self, other: &Self, t: f64) -> Self {
        Self::new(
            self.x() + t * (other.x() - self.x()),
            self.x() + t * (other.y() - self.x()),
        )
    }
    
    /// 向下取整
    #[inline]
    fn floor(&self) -> Self {
        Self::new(self.x().floor(), self.x().floor())
    }
    
    /// 向上取整
    #[inline]
    fn ceil(&self) -> Self {
        Self::new(self.x().ceil(), self.x().ceil())
    }
    
    /// 四舍五入
    #[inline]
    fn round(&self) -> Self {
        Self::new(self.x().round(), self.x().round())
    }
    
    fn to_tuple(&self) -> (f64, f64) {
        (self.x(), self.y())
    }
    
    fn from_tuple(tuple: (f64, f64)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
    
    #[inline]
    fn modify_x(self, op: impl FnOnce(f64) -> f64) -> Self {
        Self::new(op(self.x()), self.y())
    }
    
    #[inline]
    fn modify_y(self, op: impl FnOnce(f64) -> f64) -> Self {
        Self::new(self.x(), op(self.y()))
    }
    
    #[inline]
    fn modify(self, op: impl FnOnce((f64,f64)) -> (f64,f64)) -> Self {
        Self::from_tuple(op(self.to_tuple()))
    }
    
}

pub trait ToPhysicalVec {
    fn to_physical_vec(&self) -> (f32, f32);
}

pub trait FromPhysicalVec {
    fn from_physical_vec(physical: (f32, f32)) -> Self;
}

///
/// 给我一个像这样的类型
/// ```
/// pub MyVec(pub f64, pub f64);
/// ```
#[macro_export]
macro_rules! impl_vec {
    (
        $TypE:ident;
        
        $($other:tt)*
    ) => {
    
impl $crate::measure::MeasureVec for $TypE {
    /// 创建新的相对坐标
    #[inline]
    fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    #[inline]
    fn x(&self) -> f64 {
        self.0
    }
    
    #[inline]
    fn y(&self) -> f64 {
        self.1
    }
    
    $($other)*
}


impl Neg for $TypE {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Add for $TypE {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for $TypE {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for $TypE {
    type Output = Self;
    
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for $TypE {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<f64> for $TypE {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<f64> for $TypE {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl Div<f64> for $TypE {
    type Output = Self;
    
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl DivAssign<f64> for $TypE {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl From<$TypE> for (f64, f64) {
    #[inline]
    fn from(pos: $TypE) -> Self {
        (pos.0, pos.1)
    }
}

impl From<(f64, f64)> for $TypE {
    #[inline]
    fn from((x, y): (f64, f64)) -> Self {
        Self(x, y)
    }
}

impl From<$TypE> for (f32, f32) {
    #[inline]
    fn from(pos: $TypE) -> Self {
        pos.to_physical_vec()
    }
}

impl From<$TypE> for f32 {
    fn from(value: $TypE) -> Self {
        value.to_sum()
    }
}

impl From<$crate::math::Vec2> for $TypE {
        #[inline]
    fn from(vec2: $crate::math::Vec2) -> Self {
        Self::from_physical_vec((vec2.x, vec2.y))
    }
}

impl From<$TypE> for $crate::math::Vec2 {
        #[inline]
    fn from(pos: $TypE) -> Self {
        let (x, y) = pos.to_physical_vec();
        $crate::math::Vec2::new(x, y)
    }
}

impl From<$TypE> for Option<$crate::math::Vec2> {
        #[inline]
    fn from(pos: $TypE) -> Self {
        let (x, y) = pos.to_physical_vec();
        Some($crate::math::Vec2::new(x, y))
    }
}
    
    };
}
pub use impl_vec;

#[macro_export]
macro_rules! impl_position_vec_type {
    (
        $TypE:ident,
        $top_left:expr,
        $bottom_right:expr
    ) => {
        impl $TypE {
            pub const ZERO: Self = Self(0.0,0.0);
            
            pub const ONE: Self = Self(1.0,1.0);
            
            pub const LT: Self = Self($top_left.0, $top_left.1);
            pub const LB: Self = Self($top_left.0, $bottom_right.1);
            pub const RT: Self = Self($bottom_right.0, $top_left.1);
            pub const RB: Self = Self($bottom_right.0, $bottom_right.1);
            
            pub const RC: Self = Self($bottom_right.0, ($top_left.1 + $bottom_right.1) / 2.0);
            pub const LC: Self = Self($top_left.0, ($top_left.1 + $bottom_right.1) / 2.0);
            pub const CT: Self = Self(($top_left.0 + $bottom_right.0) / 2.0, $top_left.1);
            pub const CB: Self = Self(($top_left.0 + $bottom_right.0) / 2.0, $bottom_right.1);
            
            pub const CC: Self = Self(
                ($top_left.0 + $bottom_right.0) / 2.0,
                ($top_left.1 + $bottom_right.1) / 2.0
            );
        }

        $crate::impl_vec!{
            $TypE;
        }

        impl $crate::measure::ToPhysicalVec for $TypE {
            #[inline]
            fn to_physical_vec(&self) -> (f32, f32) {
                use miniquad::window::screen_size;
                use crate::measure::{get_measure_ratio, dyn_pos};
                use crate::input::mouse_position_local;

                let (screen_w, screen_h) = screen_size();
                
                let visible_w = get_measure_ratio().map_or(screen_w, |r| {
                    screen_w.min((screen_h as f64 * r) as f32)
                });
                
                let offset_x = (screen_w - visible_w) / 2.0;

                let range_x = $bottom_right.0 - $top_left.0;
                let range_y = $bottom_right.1 - $top_left.1;

                let ratio_x = if range_x != 0.0 { (self.0 - $top_left.0) / range_x } else { 0.0 };
                let ratio_y = if range_y != 0.0 { (self.1 - $top_left.1) / range_y } else { 0.0 };

                let mut physical_x = offset_x + (ratio_x as f32) * visible_w;
                let mut physical_y = (ratio_y as f32) * screen_h;

                if let Some((fx, fy)) = dyn_pos() {
                    let mouse = mouse_position_local();
                    let d = (mouse.abs() + 1.0).ln();
                    
                    physical_x -= mouse.x.signum() * d.x * fx;
                    physical_y -= mouse.y.signum() * d.y * fy;
                }

                (physical_x, physical_y)
            }
        }

        impl $crate::measure::FromPhysicalVec for $TypE {
            #[inline]
            fn from_physical_vec(physical: (f32, f32)) -> Self {
                use miniquad::window::screen_size;
                use crate::measure::{get_measure_ratio};

                let (screen_w, screen_h) = screen_size();
                
                let visible_w = get_measure_ratio().map_or(screen_w, |r| {
                    screen_w.min((screen_h as f64 * r) as f32)
                });
                
                let offset_x = (screen_w - visible_w) / 2.0;

                if visible_w == 0.0 || screen_h == 0.0 {
                    return Self::ZERO;
                }

                let ratio_x = ((physical.0 - offset_x) as f64) / (visible_w as f64);
                let ratio_y = (physical.1 as f64) / (screen_h as f64);

                let range_x = $bottom_right.0 - $top_left.0;
                let range_y = $bottom_right.1 - $top_left.1;

                let log_x = $top_left.0 + ratio_x * range_x;
                let log_y = $top_left.1 + ratio_y * range_y;

                Self(log_x, log_y)
            }
        }
    };
}



/// Down-left-right 坐标
///
/// 坐标系定义：
/// - 原点 (0,0): 屏幕左上角
/// - X轴: 向右为正
/// - Y轴: 向下为正
/// - (1.0, 1.0): 屏幕右下角
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dlt(pub f64, pub f64);

impl_position_vec_type!(Dlt, (0.0, 0.0), (1.0, 1.0));

impl From<OneUcc> for Dlt {
    #[inline]
    fn from(pos: OneUcc) -> Self {
        Dlt(
            (pos.0 + 1.0) / 2.0,
            1.0 - (pos.1 + 1.0) / 2.0
        )
    }
}

/// Up-center-center 坐标
///
/// 坐标系定义：
/// - 单位长度: 1.0 为一倍的屏幕宽或高
/// - 原点 (0,0): 屏幕中心
/// - X轴: 向右为正
/// - Y轴: 向上为正
/// - (-0.5,-0.5): 屏幕左下角
/// - ( 0.5, 0.5): 屏幕右上角
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ucc(pub f64, pub f64);

impl_position_vec_type!(Ucc, (-0.5, 0.5), (0.5, -0.5));


/// One-Up-center-center 坐标
///
/// 坐标系定义：
/// - 单位长度: 1.0 为一半的屏幕宽或高
/// - 原点 (0,0): 屏幕中心
/// - X轴: 向右为正
/// - Y轴: 向上为正
/// - (-1.0,-1.0): 屏幕左下角
/// - ( 1.0, 1.0): 屏幕右上角
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OneUcc(pub f64, pub f64);

impl_position_vec_type!(OneUcc, (-1.0, 1.0), (1.0, -1.0));

impl From<Dlt> for OneUcc {
    #[inline]
    fn from(coo: Dlt) -> Self {
        OneUcc(
            coo.0 * 2.0 - 1.0,
            1.0 - coo.1 * 2.0
        )
    }
}

/// 相对向量
///
/// n倍宽高，无偏移
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VeC(pub f64, pub f64);

impl VeC {
    pub const ZERO: Self = Self(0.0,0.0);
    pub const ONE: Self = Self(1.0,1.0);
    pub const NONE: Self = Self(0.0,0.0);
    pub const FULL: Self = Self(1.0,1.0);
    pub const HALF: Self = Self(0.5,0.5);
}

impl_vec!{
    VeC;
}

impl ToPhysicalVec for VeC {
    /// 相对大小 → 实际像素大小
    #[inline]
    fn to_physical_vec(&self) -> (f32, f32) {
        let (screen_w, screen_h) = screen_size();
        let screen_w = get_measure_ratio().map_or(screen_w, |r| screen_w.min((screen_h as f64 * r) as f32));
        let physical_x = (self.0 as f32) * screen_w;
        let physical_y = (self.1 as f32) * screen_h;
        // let delta = mouse_position_local() * 10.;
        // (physical_x-delta.x, physical_y-delta.y)
        (physical_x, physical_y)
    }
}

impl FromPhysicalVec for VeC {
    /// 实际像素大小 → 相对大小
    #[inline]
    fn from_physical_vec(physical: (f32, f32)) -> Self {
        let (screen_w, screen_h) = screen_size();
        let screen_w = get_measure_ratio().map_or(screen_w, |r|screen_w.min((screen_h as f64 * r) as f32));
        
        let x = physical.0 as f64 / screen_w as f64;
        let y = physical.1 as f64 / screen_h as f64;
        Self(x, y)
    }
}


pub fn half_area() -> (f32, f32) {
    let (screen_w, screen_h) = screen_size();
    let visible_w = get_measure_ratio().map_or(screen_w, |r|screen_w.min((screen_h as f64 * r) as f32));
    let visible_h = screen_h;
    
    (visible_w / 2.0, visible_h / 2.0)
}

pub fn screen_offset() -> f32 {
    let (screen_w, _) = screen_size();
    let (half_visible_w, _) = half_area();
    (screen_w / 2.0) - half_visible_w // 仅>16:9时≠0，≤16:9时=0
}

impl ToPhysicalVec for (f32, f32) {
    #[inline]
    fn to_physical_vec(&self) -> (f32, f32) {
        *self
    }
}

impl FromPhysicalVec for (f32, f32) {
    #[inline]
    fn from_physical_vec(physical: (f32, f32)) -> Self {
        physical
    }
}

pub type VecChain<'s> = Chain<'s, (f32, f32)>;

pub trait ToPhysical {
    fn to_physical(&self) -> f32;
}

impl<T: ToPhysicalVec> ToPhysical for T {
    fn to_physical(&self) -> f32 {
        let (x,y) = self.to_physical_vec();
        x + y
    }
}


impl<T: ToPhysical> ToPhysical for (T,T) {
    fn to_physical(&self) -> f32 {
        let (x,y) = self;
        x.to_physical() + y.to_physical()
    }
}

use crate::helper::chain::Chain;
use std::f32::consts::PI;

/// 将一个点围绕另一个基准点旋转指定角度
///
/// # 参数
/// * `point`: 待旋转的点 (实现 ToPhysicalVec)
/// * `center`: 旋转中心点 (实现 ToPhysicalVec)
/// * `angle`: 旋转角度（度数制，顺时针为正）
///
/// # 返回
/// 旋转后的物理坐标 (x, y)
pub fn rotate_pos(
    point: impl ToPhysicalVec,
    center: impl ToPhysicalVec,
    angle: f32,
) -> (f32, f32) {
    let (px, py) = point.to_physical_vec();
    let (cx, cy) = center.to_physical_vec();
    
    let dx = px - cx;
    let dy = py - cy;
    
    let radians = angle * PI / 180.0;
    let cos_r = radians.cos();
    let sin_r = radians.sin();
    
    // x' = x*cos(theta) - y*sin(theta)
    // y' = x*sin(theta) + y*cos(theta)
    let rotated_dx = dx * cos_r - dy * sin_r;
    let rotated_dy = dx * sin_r + dy * cos_r;
    
    (cx + rotated_dx, cy + rotated_dy)
}


/// 计算一个点在局部坐标系下（指定原点和旋转角度）的物理坐标
///
/// # 参数
/// * `point`: 局部坐标系中的点。**相对偏移量**。<br>
///   注意：此处建议传入表示“相对大小/位移”的类型（如 [`VeC`]）。<br>
///   ***DO NOT*** 传入包含全局偏移信息的绝对坐标（如 [`Dlt`], [`Ucc`] 等），<br>
///   否则旋转中心可能会发生预料之外的偏移。
/// * `origin`: 局部坐标系的原点在物理屏幕上的位置。
/// * `angle`: 局部坐标系的旋转角度（度数制，顺时针为正）。
///
/// # 返回
/// 该点在物理屏幕上的最终坐标 (x, y)
pub fn refer_pos(
    point: impl ToPhysicalVec,
    origin: impl ToPhysicalVec,
    angle: f32,
) -> (f32, f32) {
    let (ox, oy) = origin.to_physical_vec();
    
    let (dx, dy) = point.to_physical_vec();
    
    let rad = angle * PI / 180.0;
    let cos_r = rad.cos();
    let sin_r = rad.sin();
    
    // 新的偏移量
    let rotated_dx = dx * cos_r - dy * sin_r;
    let rotated_dy = dx * sin_r + dy * cos_r;
    
    (ox + rotated_dx, oy + rotated_dy)
}