//! This module defines the [`Roll`] helper that can check scrolling events and provide status or scolling.

use crate::prelude::*;


/// A [`Roll`] helper that can check scrolling events and provide status or scolling.
///
/// 返回值为相对单位，n倍的屏幕宽度或高度
pub struct Roll {
    /// 参考的方向。决定返回值相对于屏幕高度还是宽度
    direction: Direction,
    /// 偏移上限
    max: Option<f32>,
    /// 使用鼠标滚动时始终磁吸。
    ///
    /// 否则单词滚动固定整数倍的120.0像素（miniquad默认行为）
    mouse_always_snap: bool,
    /// 一次滚动与磁吸的间隔
    interval: Option<f32>,
    /// 磁吸
    threshold: Option<f32>,
    delta: f32,
    fixed_delta: f32,
    dragging: bool,
    last_pressed_mouse: Option<(f32,f32)>,
}

impl Default for Roll {
    fn default() -> Self {
        Self {
            max: None,
            direction: Direction::Horizontal,
            mouse_always_snap: true,
            interval: Some(0.2),
            threshold: Some(0.01),
            delta: 0.0,
            fixed_delta: 0.0,
            dragging: false,
            last_pressed_mouse: None,
        }
    }
}

impl Roll {
    /// Creates a new [`Roll`] helper.
    pub fn new(
        max: Option<f32>,
        direction: Direction,
        mouse_always_snap: bool,
        interval: Option<f32>,
        threshold: Option<f32>,
    ) -> Self {
        Self {
            max,
            direction,
            mouse_always_snap,
            interval,
            threshold,
            delta: 0.0,
            fixed_delta: 0.0,
            dragging: false,
            last_pressed_mouse: None,
        }
    }
    
    pub fn blank() -> Self {
        Self{
            max: None,
            direction: Direction::Vertical,
            mouse_always_snap: false,
            interval: None,
            threshold: None,
            delta: 0.0,
            fixed_delta: 0.0,
            dragging: false,
            last_pressed_mouse: None,
        }
    }
    
    define_with_and_fix_methods!(
        max: Option<f32>,
        direction: enum Direction {
            vertical: Vertical,
            horizontal: Horizontal,
        },
        mouse_always_snap: bool,
        interval: Option<f32>,
        threshold: Option<f32>,
    );
    
    pub fn dragging(&mut self) -> bool {
        self.dragging
    }
    
    pub fn process(&mut self) -> f32 {
        let dr = mouse_wheel();
        let mut dr = dr.0 - dr.1;
        if let Some((x,y)) = self.last_pressed_mouse {
            match self.direction {
                Direction::Horizontal => {
                    dr -= mouse_position().0 - x;
                }
                Direction::Vertical => {
                    dr -= mouse_position().1 - y;
                }
            }
            if !self.dragging && dr.abs() > 1.0{
                // println!("t");
                self.dragging = true;
            }
        }
        
        let window = match self.direction {
            Direction::Horizontal => {
                screen_width()
            }
            Direction::Vertical => {
                screen_height()
            }
        };
        
        // println!("{}", self.delta);
        // println!("{}", dr.abs());
        let interval = self.interval;
        let snap_threshold = self.threshold;
        if dr.abs() > 0.00001 {
            // 滚轮时是120.0的整数倍
            self.fixed_delta = if self.mouse_always_snap && interval.is_some() && dr.abs() % 120.0 == 0.0 {
                let interval = interval.unwrap();
                let before = (self.delta / interval).round() * interval;
                let after = before + (dr / 120.0).round() * interval;
                self.delta = Self::clamp(after, self.max);
                self.delta
            } else {
                let before = Self::clamp(self.delta + dr / window, self.max);
                self.delta = before;
                if let (Some(interval),Some(snap_threshold)) = (interval,snap_threshold) {
                    let mut after =
                        (before / interval).round() * interval;
                    let delta = (before - after);
                    if (0.0..snap_threshold).contains(&delta.abs()) {
                        after
                    } else {
                        before
                    }
                } else {
                    before
                }
            }
        };
        let dr = self.fixed_delta;
        
        self.last_pressed_mouse = if is_mouse_button_down(MouseButton::Left) {
            Some(mouse_position())
        } else {
            self.dragging = false;
            // println!("f");
            None
        };
        dr
    }
    
    fn clamp(v: f32, max: Option<f32>) -> f32{
        if let Some(max) = max {
            v.clamp(0.,max)
        } else {
            v.max(0.)
        }
    }
}
