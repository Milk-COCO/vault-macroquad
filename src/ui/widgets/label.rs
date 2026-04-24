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
    
    pub fn with_size(self, size: f32) -> Self {
        Self { size, ..self }
    }
    
    pub fn with_center(self, center: impl Into<(f32,f32)>) -> Self {
        Self { center: center.into(), ..self }
    }
    
    pub fn with_text(self, text: String) -> Self {
        Self { text, ..self }
    }
    
    pub fn with_bg(self, bg: Color) -> Self {
        Self { bg, ..self }
    }
    
    pub fn with_fg(self, fg: Color) -> Self {
        Self { fg, ..self }
    }
    
    pub fn with_font(self, font: Rc<RefCell<Font>>) -> Self {
        Self { font: Some(font), ..self }
    }
    
    pub fn with_option_font(self, font: Option<Rc<RefCell<Font>>>) -> Self {
        Self { font, ..self }
    }
    
    pub fn without_font(self) -> Self {
        Self { font: None , ..self }
    }
    
    pub fn size(&mut self, size: f32) -> &mut Self {
        self.size = size;
        self
    }
    
    pub fn center(&mut self, center: impl Into<(f32,f32)>) -> &mut Self {
        self.center = center.into();
        self
    }
    
    pub fn text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }
    
    pub fn bg(&mut self, bg: Color) -> &mut Self {
        self.bg = bg;
        self
    }
    
    pub fn fg(&mut self, fg: Color) -> &mut Self {
        self.fg = fg;
        self
    }
    
    pub fn font(&mut self, font: Rc<RefCell<Font>>) -> &mut Self {
        self.font = Some(font);
        self
    }
    
    pub fn non_font(&mut self) -> &mut Self {
        self.font = None;
        self
    }
    
    pub fn set_font(&mut self, font: Option<Rc<RefCell<Font>>>) -> &mut Self {
        self.font = font;
        self
    }
    
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
