//! This module defines the [`Button`] widget that can be clicked to perform an action.
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::*;

use super::{Widget, Action};

/// A [`Button`] widget that can be clicked to perform an action.
pub struct Button {
    size: Box<dyn ToPhysicalVec>,
    center: (f32,f32),
    text: String,
    text_color: Color,
    hovered_text_color: Color,
    bg: Color,
    fg: Color,
    font: Option<Rc<RefCell<Font>>>,
    texture: Option<Texture2D>,
    hover: bool,
    clicked: bool,
    pressed: bool,
    released: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            size: Box::new((200.0f32,50.0f32)),
            center: CTR_LT,
            text: "".to_string(),
            text_color: WHITE,
            hovered_text_color: WHITE,
            bg: DARKGRAY,
            fg: GRAY,
            font: None,
            texture: None,
            hover: false,
            clicked: false,
            pressed: false,
            released: false,
        }
    }
}

impl Button {
    /// Creates a new [`Button`] widget.
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
            clicked: false,
            pressed: false,
            released: false,
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
            font: None,
            texture: None,
            hover: false,
            clicked: false,
            pressed: false,
            released: false,
        }
    }
    
    define_with_and_fix_methods!(
        size: Box<dyn ToPhysicalVec>,
        center: (f32,f32),
        text: String,
        text_color: Color,
        hovered_text_color: Color,
        bg: Color,
        fg: Color,
        font: Option<Rc<RefCell<Font>>>,
        texture: Option<Texture2D>,
    );
    
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    
    pub fn is_released(&self) -> bool {
        self.released
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
        
        let is_hovered = mx >= x && mx <= x + size.0 && my >= y && my <= y + size.1;
        self.hover = is_hovered;
        
        if is_mouse_button_pressed(MouseButton::Left) {
            self.pressed = is_hovered;
            self.clicked = is_hovered;
            // println!("按下左键 c:{}, p:{}, h:{} ",self.clicked, self.pressed, is_hovered);
        } else {
            self.clicked = false;
        }
        
        if is_mouse_button_released(MouseButton::Left) {
            self.released = self.pressed && is_hovered;
            // println!("松开左键 r:{}, p:{}, h:{} ",self.released, self.pressed, is_hovered);
            self.pressed = false;
        } else {
            self.released = false;
        }
        
        self
    }

    fn draw(&self, pos: impl Into<(f32,f32)>){
        let size = self.size.to_physical_vec();
        let (x, y) = modify_pos_with_center(pos.into(),self.center,size);
        if let Some(texture) = &self.texture {
            draw_texture_ex(texture, (x, y), WHITE, DrawTextureParams {
                dest_size: Some(vec2(size.0,size.1)),
                ..Default::default()
            });
        }
        
        let bg = if self.hover { self.fg } else { self.bg };
        let fg = if self.hover { self.bg } else { self.fg };

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

        draw_rectangle_lines((x, y), size, 4.0, fg);
    }
}

impl Action for Button {
    fn is_clicked(&self) -> bool {
        self.clicked
    }

    fn is_hovered(&self) -> bool {
        self.hover
    }
}
