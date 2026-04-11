//! from Flowquad
//!
//! A simple, fast, and flexible library for making UI stuff with macroquad.
//!
//!
//! This is an example which shows a label and an image.
//! ```
//! use macroquad::prelude::*;
//! use flowquad::prelude::*;
//!
//! #[macroquad::main("Flowquad Example")]
//! async fn main() {
//!     let mut label = Label::new("Hello, world!".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), None, 64.0);
//!     let flowquad = load_texture("examples/flowquad.png").await.unwrap();
//!     flowquad.set_filter(FilterMode::Nearest);
//!     let mut image = flowquad::widgets::image::Image::new(256.0, 256.0, flowquad);
//!
//!     loop {
//!         clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
//!
//!         // `Label::process` Does nothing but kept here for consistency
//!         label.process(screen_width() / 2.0 - label.width() / 2.0, screen_height() / 2.0 - label.height() / 2.0 - 100.0);
//!         label.draw(screen_width() / 2.0 - label.width() / 2.0, screen_height() / 2.0 - label.height() / 2.0 - 100.0);
//!         image.process(screen_width() / 2.0 - image.width() / 2.0, screen_height() / 2.0 - image.height() / 2.0 + 50.0);
//!         image.draw(screen_width() / 2.0 - image.width() / 2.0, screen_height() / 2.0 - image.height() / 2.0 + 50.0);
//!
//!         next_frame().await;
//!     }
//! }
//! ```
//!

pub mod container;
pub mod label;
pub mod text_input;
pub mod picture;
pub mod button;
pub mod toggle;

use crate::prelude::*;
use std::any::Any;
pub use button::*;
pub use container::*;
pub use label::*;
pub use text_input::*;
pub use toggle::*;
pub use picture::*;

/// The [`Widget`] trait which defines the basic properties and methods for UI elements.
pub trait Widget: Any + IntoWidgetOption {
    /// Returns the type of the widget as an [`Any`] ref.
    fn as_any(&self) -> &dyn Any;
    /// Returns the type of the widget as an [`Any`] mut ref.
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// Returns the width of the widget.
    fn width(&self) -> f32;
    /// Returns the height of the widget.
    fn height(&self) -> f32;
    /// Returns the background color of the widget.
    fn bg(&self) -> Color;
    /// Updates the widget's state based on its position.
    fn process(&mut self, pos: impl Into<(f32,f32)>);
    /// Renders the widget at the specified position.
    fn draw(&self, pos: impl Into<(f32,f32)>);
}

/// 标记trait
/// 
/// 来标记一个具体的组件，而不是[`WidgetOption`]
pub trait Widown: Widget {}

pub trait IntoWidgetOption {
    /// Trans to [`WidgetOption`]
    fn upcast(self) -> WidgetOption;
}

/// The [`Action`] trait which defines the actions that can be performed on UI elements.
pub trait Action {
    /// Returns if the widget is clicked.
    fn is_clicked(&self) -> bool;
    /// Returns if the widget is hovered.
    fn is_hovered(&self) -> bool;
}

macro_rules! impl_widget_option {
    ($($ty:ty => $variant:ident),*) => {
        
        pub enum WidgetOption{
            $($variant($ty)),*
        }

        impl WidgetOption {
            /// 转成 &dyn Any
            pub fn as_any(&self) -> &dyn std::any::Any {
                match self {
                    $(Self::$variant(v) => v),*
                }
            }
            
            /// 转成 &mut dyn Any
            pub fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                match self {
                    $(Self::$variant(v) => v),*
                }
            }
            
            /// 下落到具体的组件类型
            pub fn downcast_ref<T:'static + Widown>(&self) -> Option<&T>{
                self.as_any().downcast_ref::<T>()
            }
            
            /// 下落到具体的组件类型
            pub fn downcast_mut<T:'static + Widown>(&mut self) -> Option<&mut T> {
                self.as_any_mut().downcast_mut::<T>()
            }
        }
        
        impl IntoWidgetOption for WidgetOption {
            fn upcast(self) -> WidgetOption {
                self
            }
        }
        
        impl Widget for WidgetOption {
            fn as_any(&self) -> &(dyn Any + 'static) { 
                self.as_any()
            }
            
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self.as_any_mut()
            }
            
            fn width(&self) -> f32 {
                match self {
                    $(Self::$variant(v) => v.width()),*
                }
            }
            
            fn height(&self) -> f32 {
                match self {
                    $(Self::$variant(v) => v.height()),*
                }
            }
            
            fn bg(&self) -> Color {
                match self {
                    $(Self::$variant(v) => v.bg()),*
                }
            }
            
            fn process(&mut self, pos: impl Into<(f32, f32)>) {
                let pos = pos.into();
                match self {
                    $(Self::$variant(v) => v.process(pos)),*
                }
            }
            
            fn draw(&self, pos: impl Into<(f32, f32)>) {
                let pos = pos.into();
                match self {
                    $(Self::$variant(v) => v.draw(pos)),*
                }
            }
        }

        $(
            impl Widown for $ty {}
        
            impl IntoWidgetOption for $ty {
                fn upcast(self) -> WidgetOption {
                    self.into()
                }
            }
            
            impl From<$ty> for WidgetOption {
                fn from(val: $ty) -> Self {
                    WidgetOption::$variant(val)
                }
            }
        )*
    };
}

impl_widget_option!(
    Container => Container,
    Label => Label,
    TextInput => TextInput,
    Picture => Image,
    Button => Button,
    Toggle => Toggle
);