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
    center: (f32,f32),
    text: String,
    text_color: Color,
    hovered_text_color: Color,
    bg: Color,
    fg: Color,
    hover: bool,
    click: bool,
    font: Option<Rc<RefCell<Font>>>,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            width: 200.0,
            height: 50.0,
            center: CTR_LT,
            text: "".to_string(),
            text_color: WHITE,
            hovered_text_color: WHITE,
            bg: DARKGRAY,
            fg: GRAY,
            hover: false,
            click: false,
            font: None,
        }
    }
}

impl Button {
    /// Creates a new [`Button`] widget.
    pub fn new(width: f32, height: f32, center: impl Into<(f32,f32)>, text: String, text_color: Color, hovered_text_color: Color, bg: Color, fg: Color, font: Option<Rc<RefCell<Font>>>) -> Self {
        Self {
            width,
            height,
            center: center.into(),
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
    
    pub fn with_width(self, width: f32) -> Self {
        Self { width, ..self }
    }
    
    pub fn with_height(self, height: f32) -> Self {
        Self { height, ..self }
    }
    
    pub fn with_size(self, size: impl Into<(f32,f32)>) -> Self {
        let (width, height) = size.into();
        Self { width, height, ..self }
    }
    
    pub fn with_center(self, center: impl Into<(f32,f32)>) -> Self {
        Self { center: center.into(), ..self }
    }
    
    pub fn with_text(self, text: String) -> Self {
        Self { text, ..self }
    }
    
    pub fn with_text_color(self, text_color: Color) -> Self {
        Self { text_color, ..self }
    }
    
    pub fn with_hovered_text_color(self, hovered_text_color: Color) -> Self {
        Self { hovered_text_color, ..self }
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
    
    
    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }
    
    pub fn height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }
    
    pub fn size(&mut self, size: impl Into<(f32,f32)>) -> &mut Self {
        let (width, height) = size.into();
        self.width = width;
        self.height = height;
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
    
    pub fn text_color(&mut self, text_color: Color) -> &mut Self {
        self.text_color = text_color;
        self
    }
    
    pub fn hovered_text_color(&mut self, hovered_text_color: Color) -> &mut Self {
        self.hovered_text_color = hovered_text_color;
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

    fn process(&mut self, pos: impl Into<(f32,f32)>) -> &mut Self {
        let (x, y) = modify_pos_with_center(pos.into(),self.center,(self.width,self.height));
        let mouse_pos = mouse_position();
        let mx = mouse_pos.0;
        let my = mouse_pos.1;

        self.hover = mx >= x && mx <= x + self.width && my >= y && my <= y + self.height;
        self.click = self.hover && is_mouse_button_pressed(MouseButton::Left);
        self
    }

    fn draw(&self, pos: impl Into<(f32,f32)>){
        let (x, y) = modify_pos_with_center(pos.into(),self.center,(self.width,self.height));
        
        let bg = if self.hover { self.fg } else { self.bg };
        let fg = if self.hover { self.bg } else { self.fg };

        draw_rectangle((x, y), (self.width, self.height), bg);
        
        let size = self.height * 0.4;
        
        let text_color = if !self.hover { self.text_color } else { self.hovered_text_color };
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
