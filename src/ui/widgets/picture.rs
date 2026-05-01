//! This module defines the [`Picture`] widget that displays an image on the screen.
use std::any::Any;
use crate::prelude::*;

use super::Widget;

/// The [`Picture`] widget that displays an image on the screen.
pub struct Picture {
    size: Box<dyn ToPhysicalVec>,
    center: (f32,f32),
    texture: Texture2D
}

impl Default for Picture {
    fn default() -> Self {
        Self {
            size: Box::new((100.0f32,1000.0f32)),
            center: CTR_LT,
            texture: Texture2D::empty(),
        }
    }
}

impl Picture {
    /// Creates a new [`Picture`] widget.
    pub fn new(
        size: impl ToPhysicalVec + 'static,
        center: impl Into<(f32,f32)>,
        texture: Texture2D
    ) -> Self {
        Self {
            size: Box::new(size),
            center: center.into(),
            texture
        }
    }
    
    pub fn with_size(self, size: impl ToPhysicalVec + 'static) -> Self {
        Self { size: Box::new(size), ..self }
    }
    
    pub fn with_center(self, center: impl Into<(f32,f32)>) -> Self {
        Self { center: center.into(), ..self }
    }
    
    pub fn with_texture(self, texture: Texture2D) -> Self {
        Self { texture, ..self }
    }
    
    pub fn size(&mut self, size: impl ToPhysicalVec + 'static) -> &mut Self {
        self.size = Box::new(size);
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
        self.size.to_physical_vec().0
    }
    
    fn height(&self) -> f32 {
        self.size.to_physical_vec().1
    }
    
    fn bg(&self) -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    fn process(&mut self, _pos: impl Into<(f32,f32)>) -> &mut Self {
        // Nothing :D
        self
    }

    fn draw(&self, pos: impl Into<(f32,f32)>) {
        let size = self.size.to_physical_vec();
        let (x, y) = modify_pos_with_center(pos.into(),self.center,size.into());
        draw_texture_ex(&self.texture,(x,y), WHITE, DrawTextureParams {
            dest_size: Some(vec2(size.0,size.1)),
            ..Default::default()
        });
    }
}
