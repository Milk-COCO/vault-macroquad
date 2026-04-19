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
    #[cfg(not(target_os = "Android"))]
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

pub trait Measure
where Self: Sized
{
    #[inline]
    fn new(x: f64, y: f64) -> Self;
    
    fn to_physical(&self) -> (f32, f32) ;
    
    fn from_physical(physical: (f32, f32)) -> Self ;
    
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
    
    #[inline]
    fn x(&self) -> f64;
    
    #[inline]
    fn y(&self) -> f64;
    
    /// 实际像素大小之和
    #[inline]
    fn to_sum(&self) -> f32 {
        let phy = self.to_physical();
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
    let (x, y) = self.to_physical();
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

///
/// 给我一个像这样的类型
/// ```
/// pub MyVec(pub f64, pub f64);
/// ```
///
/// 并定义这两个函数
///
/// ```
/// fn to_physical(&self) -> (f32, f32) {
///     $($tp)*
/// }
///
/// fn from_physical(physical: (f32, f32)) -> Self {
///     $($fp)*
/// }
/// ```
#[macro_export]
macro_rules! impl_measure {
    (
        $TypE:ident;
        
        $($other:tt)*
    ) => {
    
impl $crate::measure::Measure for $TypE {
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
        pos.to_physical()
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
        Self::from_physical((vec2.x, vec2.y))
    }
}

impl From<$TypE> for $crate::math::Vec2 {
        #[inline]
    fn from(pos: $TypE) -> Self {
        let (x, y) = pos.to_physical();
        $crate::math::Vec2::new(x, y)
    }
}

impl From<$TypE> for Option<$crate::math::Vec2> {
        #[inline]
    fn from(pos: $TypE) -> Self {
        let (x, y) = pos.to_physical();
        Some($crate::math::Vec2::new(x, y))
    }
}
    
    };
}
pub use impl_measure;
use crate::input::mouse_position_local;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VeC(pub f64, pub f64);

impl VeC {
    pub const ZERO: Self = Self(0.0,0.0);
    pub const ONE: Self = Self(1.0,1.0);
    pub const NONE: Self = Self(0.0,0.0);
    pub const FULL: Self = Self(1.0,1.0);
    pub const HALF: Self = Self(0.5,0.5);
}

impl_measure!{
    VeC;
    
    /// 相对大小 → 实际像素大小
    #[inline]
    fn to_physical(&self) -> (f32, f32) {
        let (screen_w, screen_h) = screen_size();
        let screen_w = get_measure_ratio().map_or(screen_w, |r|screen_w.min((screen_h as f64 * r) as f32));
        let physical_x = (self.0 as f32) * screen_w;
        let physical_y = (self.1 as f32) * screen_h;
        // let delta = mouse_position_local() * 10.;
        // (physical_x-delta.x, physical_y-delta.y)
        (physical_x, physical_y)
    }
    
    /// 实际像素大小 → 相对大小
    #[inline]
    fn from_physical(physical: (f32, f32)) -> Self {
        let (screen_w, screen_h) = screen_size();
        let screen_w = get_measure_ratio().map_or(screen_w, |r|screen_w.min((screen_h as f64 * r) as f32));
        
        // 反向逻辑：(物理坐标 - 中心坐标) / 半屏尺寸 = 相对坐标（±1.0范围）
        let x = physical.0 as f64 / screen_w as f64;
        let y = physical.1 as f64 / screen_h as f64;
        Self(x, y)
    }
}

/// 相对屏幕中的坐标
/// - 元组结构体：PoS(f64, f64)，直接创建更便捷
/// - 0.0 → 屏幕中心，1.0 → 右/下边界，-1.0 → 左/上边界（核心修改：±1.0 对应边界）
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PoS(pub f64, pub f64);

impl PoS {
    pub const LT: Self = Self(-1.0, 1.0);
    pub const LB: Self = Self(-1.0, -1.0);
    pub const RT: Self = Self(1.0, 1.0);
    pub const RB: Self = Self(1.0, -1.0);
    pub const RC: Self = Self(1.0, 0.0);
    pub const LC: Self = Self(-1.0, 0.0);
    pub const CT: Self = Self(0.0, 1.0);
    pub const CB: Self = Self(0.0, -1.0);
    pub const CC: Self = Self(0.0,0.0);
    
    pub const ZERO: Self = Self(0.0,0.0);
    pub const C: Self = Self(0.0,0.0);
    
    fn half_area() -> (f32, f32) {
        let (screen_w, screen_h) = screen_size();
        let visible_w = get_measure_ratio().map_or(screen_w, |r|screen_w.min((screen_h as f64 * r) as f32));
        let visible_h = screen_h;
        
        (visible_w / 2.0, visible_h / 2.0)
    }
    
    fn screen_offset() -> f32 {
        let (screen_w, _) = screen_size();
        let (half_visible_w, _) = Self::half_area();
        (screen_w / 2.0) - half_visible_w // 仅>16:9时≠0，≤16:9时=0
    }
}

impl_measure!{
    PoS;
    
    #[inline]
    fn to_physical(&self) -> (f32, f32) {
        let (half_w, half_h) = Self::half_area();
        let offset = Self::screen_offset();
    
        let physical_x = half_w + (self.0 as f32) * half_w + offset;
        let physical_y = half_h - (self.1 as f32) * half_h;
        
        if let Some((fx,fy)) = dyn_pos() {
            let mouse = mouse_position_local();
            let d = (mouse.abs() + 1.0).ln() ;
            (physical_x-mouse.x.signum() * d.x * fx, physical_y-mouse.y.signum() * d.y * fy)
        } else {
            (physical_x, physical_y)
        }
    }
    
    #[inline]
    fn from_physical(physical: (f32, f32)) -> Self {
        let (half_w, half_h) = Self::half_area();
        let offset = Self::screen_offset();
    
        let visible_x = physical.0 - offset;
    
        let x = (visible_x - half_w) as f64 / half_w as f64;
        let y = -(physical.1 - half_h) as f64 / half_h as f64;
    
        Self(x, y)
    }
}