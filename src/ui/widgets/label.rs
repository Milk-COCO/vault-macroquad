//! This module defines the [`Label`] widget that displays text on the screen.
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::*;

use super::Widget;

/// The [`Label`] widget that displays text on the screen.
pub struct Label {
    text: String,
    center: (f32,f32),
    bg: Color,
    fg: Color,
    font: Option<Rc<RefCell<Font>>>,
    size: f32,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            center: CTR_LT,
            text: "".to_string(),
            bg: DARKGRAY,
            fg: GRAY,
            font: None,
            size: 10.0,
        }
    }
}

impl Label {
    /// Creates a new [`Label`] widget.
    pub fn new(text: String, center: impl Into<(f32,f32)>, bg: Color, fg: Color, font: Option<Rc<RefCell<Font>>>, size: f32) -> Self {
        Self {
            text,
            center: center.into(),
            bg,
            fg,
            font,
            size,
        }
    }
    
    define_with_and_fix_methods!(
        size: f32,
        center: (f32,f32),
        text: String,
        bg: Color,
        fg: Color,
        font: Option<Rc<RefCell<Font>>>,
    );
    
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Widget for Label {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn width(&self) -> f32 {
        let size = self.height();
        let text_size = measure_text(&self.text, self.font.clone(), size, 1.0);
        text_size.width
    }

    fn height(&self) -> f32 {
        self.size
    }

    fn bg(&self) -> Color {
        self.bg
    }

    fn process(&mut self, _pos: impl Into<(f32, f32)>) -> &mut Self {
        // Nothing :D
        self
    }

    fn draw(&self, pos: impl Into<(f32, f32)>) {
        let size = self.size;
        let text_size = measure_text(&self.text, self.font.clone(), size, 1.0);
        let width = text_size.width;
        let height = text_size.height;
        
        let (x, y) = modify_pos_with_center(pos.into(),self.center,(width,height));
        let bg = self.bg;
        let fg = self.fg;

        draw_rectangle((x,y), (width, height), bg);
        draw_text_ex(&self.text,
            (x , y + text_size.height),
            (-1.,-1.),
            TextParams {
                font: self.font.clone(),
                font_size: size,
                font_scale: 1.0,
                color: fg,
                ..Default::default()
            }
        );
    }
}
