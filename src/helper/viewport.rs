use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::{impl_vec, thread_assert};
use crate::prelude::*;
use crate::measure::ToPhysicalVec;

/// 游戏世界坐标类型
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GamePoS(pub f64, pub f64);

impl GamePoS {
    pub const HALF_W: f64 = 0.5;
    pub const HALF_H: f64 = 0.5;
}

/// 定义游戏视口矩形，由两个 坐标 决定
pub struct GameViewport {
    pub left_top: Box<dyn ToPhysicalVec>,
    pub right_bottom: Box<dyn ToPhysicalVec>,
}

impl GameViewport {
    pub fn new(top_left: impl ToPhysicalVec + 'static, bottom_right: impl ToPhysicalVec + 'static) -> Self {
        Self {
            left_top: Box::new(top_left),
            right_bottom: Box::new(bottom_right),
        }
    }

    /// 获取视口在屏幕上的像素参数 (x, y, w, h)
    #[inline]
    pub fn to_screen_rect(&self) -> (f32, f32, f32, f32) {
        let (tl_x, tl_y) = self.left_top.to_physical_vec();
        let (br_x, br_y) = self.right_bottom.to_physical_vec();

        let w = br_x - tl_x;
        let h = br_y - tl_y;

        let (w, h) = if w < 0.0 && h < 0.0 { (-w, -h) } else { (w.abs(), h.abs()) };

        (tl_x, tl_y, w, h)
    }

    /// 获取视口的尺寸 (width, height)
    pub fn get_size(&self) -> (f32, f32) {
        let (..,w,h) = self.to_screen_rect();
        (w,h)
    }

    /// 获取视口的左上角坐标 (x, y)
    pub fn get_offset(&self) -> (f32, f32) {
        let (x,y,..) = self.to_screen_rect();
        (x,y)
    }

    /// 获取视口中心绝对坐标
    pub fn get_center(&self) -> (f32, f32) {
        let (x,y,w,h) = self.to_screen_rect();
        (x+w/2.,y+h/2.)
    }
    
    /// 将视口内的相对像素坐标转换为屏幕绝对坐标
    ///
    /// relative: 相对于视口中心的像素偏移 (dx, dy)
    /// 注意传入相对坐标向上为正。
    pub fn relative_to_absolute(&self, relative: (f32, f32)) -> (f32, f32) {
        let (fx, fy, fw, fh) = self.to_screen_rect();
        (fx + relative.0 + fw/2., fy - relative.1 + fh/2.)
    }
}

static mut GAME_VIEWPORT: Option<GameViewport> = None;

/// 设置游戏视口
pub fn set_game_viewport(viewport: GameViewport) {
    thread_assert::same_thread();
    unsafe {
        GAME_VIEWPORT = Some(viewport);
    }
}

/// 获取当前游戏视口
#[allow(static_mut_refs)]
pub fn get_viewport() -> &'static mut GameViewport {
    thread_assert::same_thread();
    unsafe {
        GAME_VIEWPORT.as_mut().unwrap()
    }
}

/// 获取当前游戏视口单位像素
#[allow(static_mut_refs)]
pub fn get_viewport_screen() -> (f32, f32, f32, f32) {
    thread_assert::same_thread();
    unsafe {
        GAME_VIEWPORT.as_ref().unwrap().to_screen_rect()
    }
}

/// 获取当前游戏视口的尺寸 (width, height)
#[allow(static_mut_refs)]
pub fn get_viewport_size() -> (f32, f32) {
    thread_assert::same_thread();
    unsafe {
        GAME_VIEWPORT.as_ref().unwrap().get_size()
    }
}

/// 获取当前游戏视口的尺寸 (width, height)
#[allow(static_mut_refs)]
pub fn get_viewport_offset() -> (f32, f32) {
    thread_assert::same_thread();
    unsafe {
        GAME_VIEWPORT.as_ref().unwrap().get_offset()
    }
}

#[allow(static_mut_refs)]
pub fn get_viewport_center() -> (f32, f32) {
    thread_assert::same_thread();
    unsafe {
        GAME_VIEWPORT.as_ref().unwrap().get_center()
    }
}

/// 将视口内的相对像素坐标转换为屏幕绝对坐标 (全局单例快捷方法)
/// relative: 相对于视口中心的像素偏移 (dx, dy)
pub fn viewport_relative_to_absolute(relative: (f32, f32)) -> (f32, f32) {
    thread_assert::same_thread();
    unsafe {
        GAME_VIEWPORT.as_ref().unwrap().relative_to_absolute(relative)
    }
}

impl_vec!{
    GamePoS;
}

impl ToPhysicalVec for GamePoS {
    #[inline]
    fn to_physical_vec(&self) -> (f32, f32) {
        let viewport = get_viewport();
        let (fx, fy, fw, fh) = viewport.to_screen_rect();

        let rel_x = self.0 / Self::HALF_W;
        let rel_y = self.1 / Self::HALF_H;

        let center_px_x = fx + fw / 2.0;
        let center_px_y = fy + fh / 2.0;

        let phys_x = center_px_x + (rel_x as f32) * (fw / 2.0);

        let phys_y = center_px_y - (rel_y as f32) * (fh / 2.0);

        (phys_x, phys_y)
    }
}

impl FromPhysicalVec for GamePoS {
    #[inline]
    fn from_physical_vec(physical: (f32, f32)) -> Self {
        let viewport = get_viewport();
        let (fx, fy, fw, fh) = viewport.to_screen_rect();

        let center_px_x = fx + fw / 2.0;
        let center_px_y = fy + fh / 2.0;

        let offset_px_x = physical.0 - center_px_x;
        let offset_px_y = center_px_y - physical.1;

        let half_fw = fw / 2.0;
        let half_fh = fh / 2.0;

        let rel_x = if half_fw > 0.0 { offset_px_x / half_fw } else { 0.0 };
        let rel_y = if half_fh > 0.0 { offset_px_y / half_fh } else { 0.0 };

        Self(rel_x as f64 * Self::HALF_W, rel_y as f64 * Self::HALF_H)
    }
}

impl GamePoS {
    /// 检查给定的屏幕像素坐标是否落在当前游戏视口矩形内
    pub fn is_pixel_in_viewport(pixel: (f32, f32)) -> bool {
        let viewport = get_viewport();
        let (fx, fy, fw, fh) = viewport.to_screen_rect();

        pixel.0 >= fx && pixel.0 <= fx + fw &&
            pixel.1 >= fy && pixel.1 <= fy + fh
    }

    /// 获取当前视口在屏幕上的矩形区域 (x, y, w, h)
    pub fn get_viewport_rect() -> (f32, f32, f32, f32) {
        let viewport = get_viewport();
        viewport.to_screen_rect()
    }
}