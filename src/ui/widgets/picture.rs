//! This module defines the [`Picture`] widget that displays an image on the screen.
use std::any::Any;

use crate::prelude::*;

use super::Widget;

/// The [`Picture`] widget that displays an image on the screen.
pub struct Picture {
    height: f32,
    width: f32,
    texture: Texture2D
}

impl Picture {
    /// Creates a new [`Picture`] widget.
    pub fn new(height: f32, width: f32, texture: Texture2D) -> Self {
        Self {
            height,
            width,
            texture
        }
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

    fn process(&mut self, _pos: impl Into<(f32,f32)>) {
        // Nothing :D
    }

    fn draw(&self, pos: impl Into<(f32,f32)>) {
        draw_texture_ex(&self.texture,pos, WHITE, DrawTextureParams {
            dest_size: Some(vec2(self.width, self.height)),
            ..Default::default()
        });
    }
}
