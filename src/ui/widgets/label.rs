//! This module defines the [`Label`] widget that displays text on the screen.
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::*;

use super::Widget;

/// The [`Label`] widget that displays text on the screen.
pub struct Label {
    text: String,
    bg: Color,
    fg: Color,
    font: Option<Rc<RefCell<Font>>>,
    size: f32,
}

impl Label {
    /// Creates a new [`Label`] widget.
    pub fn new(text: String, bg: Color, fg: Color, font: Option<Rc<RefCell<Font>>>, size: f32) -> Self {
        Self {
            text,
            bg,
            fg,
            font,
            size,
        }
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

    fn process(&mut self, _pos: impl Into<(f32, f32)>) {
        // Nothing :D
    }

    fn draw(&self, pos: impl Into<(f32, f32)>) {
        let pos = pos.into();
        let bg = self.bg;
        let fg = self.fg;

        let size = self.height();
        
        let text_size = measure_text(&self.text, self.font.clone(), size, 1.0);
        draw_rectangle(pos, (self.width(), self.height()), bg);
        draw_text_ex(&self.text,
            (pos.0, pos.1 + text_size.height),
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
