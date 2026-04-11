//! This module defines the [`Toggle`] widget that can be toggled on and off.
use std::any::Any;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::prelude::*;

use super::{Widget, Action};

/// A [`Toggle`] widget that once clicked, toggles its state between on and off.
pub struct Toggle {
    width: f32,
    height: f32,
    text: String,
    bg: Color,
    fg: Color,
    hover: bool,
    toggle: bool,
    just_clicked: bool,
    font: Option<Arc<RwLock<Font>>>,
}

impl Toggle {
    /// Creates a new [`Toggle`] widget.
    pub fn new(width: f32, height: f32, text: String, bg: Color, fg: Color, font: Option<Arc<RwLock<Font>>>) -> Self {
        Self {
            width,
            height,
            text,
            bg,
            fg,
            hover: false,
            toggle: false,
            just_clicked: false,
            font,
        }
    }
}

impl Widget for Toggle {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn bg(&self) -> Color {
        self.bg
    }

    fn process(&mut self, pos: impl Into<(f32,f32)>) {
        let (x, y) = pos.into();
        let mouse_pos = mouse_position();
        let mx = mouse_pos.0;
        let my = mouse_pos.1;

        self.hover = mx >= x && mx <= x + self.width && my >= y && my <= y + self.height;
        self.just_clicked = self.hover && is_mouse_button_pressed(MouseButton::Left);
        self.toggle = if self.just_clicked { !self.toggle } else { self.toggle };
    }

    fn draw(&self, pos: impl Into<(f32,f32)>) {
        let (x, y) = pos.into();
        let bg = if self.hover || self.toggle { self.fg } else { self.bg };
        let fg = if self.hover || self.toggle { self.bg } else { self.fg };

        draw_rectangle((x, y), (self.width, self.height), bg);
        
        let size = self.height * 0.4;
        
        let text_size = measure_text(&self.text, self.font.clone(), size, 1.0);
        draw_text_ex(&self.text,
            (x + self.width / 2.0 - text_size.width / 2.0, y + self.height / 2.0 + text_size.height / 4.0),
                     (-1.,-1.),
            TextParams {
                font: self.font.clone(),
                font_size: size,
                font_scale: 1.0,
                color: fg,
                ..Default::default()
            }
        );

        draw_rectangle_lines((x, y), (self.width, self.height), 4.0, fg);
    }
}

impl Action for Toggle {
    fn is_clicked(&self) -> bool {
        self.just_clicked
    }

    fn is_hovered(&self) -> bool {
        self.hover
    }
}
