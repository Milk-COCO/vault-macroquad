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
    
    define_with_and_fix_methods!(
        size: Box<dyn ToPhysicalVec>,
        center: (f32,f32),
        texture: Texture2D,
    );
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

    fn process(&mut self, _pos: impl ToPhysicalVec) -> &mut Self {
        // Nothing :D
        self
    }

    fn draw(&self, pos: impl ToPhysicalVec) {
        let size = self.size.to_physical_vec();
        let (x, y) = modify_pos_with_center(pos.to_physical_vec(),self.center,size.into());
        draw_texture_ex(&self.texture,(x,y), WHITE, DrawTextureParams {
            dest_size: Some(vec2(size.0,size.1)),
            ..Default::default()
        });
    }
}
