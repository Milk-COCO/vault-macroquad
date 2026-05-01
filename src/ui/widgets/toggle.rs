//! This module defines the [`Toggle`] widget that can be toggled on and off.
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::*;

use super::{Widget, Action};

/// A [`Toggle`] widget that once clicked, toggles its state between on and off.
pub struct Toggle {
    size: Box<dyn ToPhysicalVec>,
    center: (f32,f32),
    text: String,
    text_color: Color,
    hovered_text_color: Color,
    bg: Color,
    fg: Color,
    hover: bool,
    toggle: bool,
    just_clicked: bool,
    font: Option<Rc<RefCell<Font>>>,
    texture: Option<Texture2D>
}

impl Default for Toggle {
    fn default() -> Self {
        Self {
            size: Box::new((200.0f32,50.0f32)),
            center: CTR_LT,
            text: "".to_string(),
            text_color: WHITE,
            hovered_text_color: WHITE,
            bg: DARKGRAY,
            fg: GRAY,
            hover: false,
            toggle: false,
            font: None,
            just_clicked: false,
            texture: None,
        }
    }
}

impl Toggle {
    /// Creates a new [`Toggle`] widget.
    pub fn new(
        size: impl ToPhysicalVec + 'static,
        center: impl Into<(f32,f32)>,
        text: String,
        text_color: Color,
        hovered_text_color: Color,
        bg: Color,
        fg: Color,
        font: Option<Rc<RefCell<Font>>>,
        texture: Option<Texture2D>,
    ) -> Self {
        Self {
            size: Box::new(size),
            center: center.into(),
            text,
            text_color,
            hovered_text_color,
            bg,
            fg,
            hover: false,
            toggle: false,
            just_clicked: false,
            font,
            texture,
        }
    }
    
    pub fn blank() -> Self {
        Self{
            size: Box::new((200.0f32,50.0f32)),
            center: CTR_LT,
            text: "".to_string(),
            text_color: BLANK,
            hovered_text_color: BLANK,
            bg: BLANK,
            fg: BLANK,
            hover: false,
            toggle: false,
            font: None,
            just_clicked: false,
            texture: None,
        }
    }
    
    pub fn with_size(self, size: impl ToPhysicalVec + 'static) -> Self {
        Self { size: Box::new(size), ..self }
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
    
    pub fn with_texture(self, texture: Texture2D) -> Self {
        Self { texture: Some(texture), ..self }
    }
    
    pub fn with_option_texture(self, texture: Option<Texture2D>) -> Self {
        Self { texture, ..self }
    }
    
    pub fn without_texture(self) -> Self {
        Self { texture: None, ..self }
    }
    
    pub fn size(&mut self, size: impl ToPhysicalVec + 'static) -> &mut Self {
        self.size = Box::new(size);
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
    
    pub fn texture(&mut self, texture: Texture2D) -> &mut Self {
        self.texture = Some(texture);
        self
    }
    
    pub fn non_texture(&mut self) -> &mut Self {
        self.texture = None;
        self
    }
    
    pub fn set_texture(&mut self, texture: Option<Texture2D>) -> &mut Self {
        self.texture = texture;
        self
    }
    
    
    pub fn get_text(&self) -> String {
        self.text.clone()
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
        self.size.to_physical_vec().0
    }
    
    fn height(&self) -> f32 {
        self.size.to_physical_vec().1
    }


    fn bg(&self) -> Color {
        self.bg
    }

    fn process(&mut self, pos: impl Into<(f32,f32)>) -> &mut Self {
        let size = self.size.to_physical_vec();
        let (x, y) = modify_pos_with_center(pos.into(),self.center,size);
        let mouse_pos = mouse_position();
        let mx = mouse_pos.0;
        let my = mouse_pos.1;

        self.hover = mx >= x && mx <= x + size.0 && my >= y && my <= y + size.1;
        self.just_clicked = self.hover && is_mouse_button_pressed(MouseButton::Left);
        self.toggle = if self.just_clicked { !self.toggle } else { self.toggle };
        self
    }

    fn draw(&self, pos: impl Into<(f32,f32)>) {
        let size = self.size.to_physical_vec();
        let (x, y) = modify_pos_with_center(pos.into(),self.center,size);
        if let Some(texture) = &self.texture {
            draw_texture_ex(texture, (x, y), WHITE, DrawTextureParams {
                dest_size: Some(vec2(size.0,size.1)),
                ..Default::default()
            });
        }
        
        let bg = if self.hover || self.toggle { self.fg } else { self.bg };
        let fg = if self.hover || self.toggle { self.bg } else { self.fg };

        draw_rectangle((x, y), size, bg);
        
        let font_size = size.1 * 0.4;
        
        let text_color = if !self.hover { self.text_color } else { self.hovered_text_color };
        let text_size = measure_text(&self.text, self.font.clone(), font_size, 1.0);
        draw_text_ex(
            &self.text,
            (x + size.0 / 2.0 - text_size.width / 2.0, y + size.1 / 2.0 + text_size.height / 4.0),
            (-1.,-1.),
            TextParams {
                font: self.font.clone(),
                font_size,
                font_scale: 1.0,
                color: text_color,
                ..Default::default()
            }
        );

        draw_rectangle_lines((x, y), (size.0, size.1), 4.0, fg);
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
