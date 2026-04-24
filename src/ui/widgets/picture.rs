//! This module defines the [`Picture`] widget that displays an image on the screen.
use std::any::Any;
use crate::prelude::*;

use super::Widget;

/// The [`Picture`] widget that displays an image on the screen.
pub struct Picture {
    height: f32,
    width: f32,
    center: (f32,f32),
    texture: Texture2D
}

impl Default for Picture {
    fn default() -> Self {
        Self {
            height: 100.0,
            width: 100.0,
            center: CTR_LT,
            texture: Texture2D::empty(),
        }
    }
}

impl Picture {
    /// Creates a new [`Picture`] widget.
    pub fn new(height: f32, width: f32, center: impl Into<(f32,f32)>, texture: Texture2D) -> Self {
        Self {
            height,
            width,
            center: center.into(),
            texture
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
    
    pub fn with_texture(self, texture: Texture2D) -> Self {
        Self { texture, ..self }
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
    
    pub fn texture(&mut self, texture: Texture2D) -> &mut Self {
        self.texture = texture;
        self
    }
}

impl Widget for Picture {
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
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    fn process(&mut self, _pos: impl Into<(f32,f32)>) -> &mut Self {
        // Nothing :D
        self
    }

    fn draw(&self, pos: impl Into<(f32,f32)>) {
        let size = vec2(self.width, self.height);
        let (x, y) = modify_pos_with_center(pos.into(),self.center,size.into());
        draw_texture_ex(&self.texture,(x,y), WHITE, DrawTextureParams {
            dest_size: Some(size),
            ..Default::default()
        });
    }
}
