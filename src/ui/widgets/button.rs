//! This module defines the [`Button`] widget that can be clicked to perform an action.
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::*;

use super::{Widget, Action};

/// A [`Button`] widget that can be clicked to perform an action.
pub struct Button {
    width: f32,
    height: f32,
    pub text: String,
    text_color: Color,
    hovered_text_color: Color,
    bg: Color,
    fg: Color,
    hover: bool,
    click: bool,
    font: Option<Rc<RefCell<Font>>>,
}

impl Button {
    /// Creates a new [`Button`] widget.
    pub fn new(width: f32, height: f32, text: String, text_color: Color, hovered_text_color: Color, bg: Color, fg: Color, font: Option<Rc<RefCell<Font>>>) -> Self {
        Self {
            width,
            height,
            text,
            text_color,
            hovered_text_color,
            bg,
            fg,
            hover: false,
            click: false,
            font,
        }
    }
    
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Widget for Button {
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
        self.click = self.hover && is_mouse_button_pressed(MouseButton::Left);
    }

    fn draw(&self, pos: impl Into<(f32,f32)>) {
        let (x, y) = pos.into();
        let bg = if self.hover { self.fg } else { self.bg };
        let fg = if self.hover { self.bg } else { self.fg };
        let text_color = if self.hover { self.text_color } else { self.hovered_text_color };

        draw_rectangle((x, y), (self.width, self.height), bg);
        
        let size = (self.height * 0.4);
        
        let text_size = measure_text(&self.text, self.font.clone(), size, 1.0);
        draw_text_ex(
            &self.text,
            (x + self.width / 2.0 - text_size.width / 2.0, y + self.height / 2.0 + text_size.height / 4.0),
            (-1.,-1.),
            TextParams {
                font: self.font.clone(),
                font_size: size,
                font_scale: 1.0,
                color: text_color,
                ..Default::default()
            }
        );

        draw_rectangle_lines((x, y), (self.width, self.height), 4.0, fg);
    }
}

impl Action for Button {
    fn is_clicked(&self) -> bool {
        self.click
    }

    fn is_hovered(&self) -> bool {
        self.hover
    }
}
