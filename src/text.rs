//! Functions to load fonts and draw text.

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::{once};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::{color::Color, get_context, get_quad_context, math::{vec3, Rect}, texture::{Image, TextureHandle}, Error};

use crate::color::WHITE;
use glam::vec2;

pub(crate) mod atlas;

use atlas::{Atlas, SpriteKey};

#[derive(Debug, Clone)]
pub(crate) struct CharacterInfo {
    pub offset_x: i32,
    pub offset_y: i32,
    pub advance: f32,
    pub sprite: SpriteKey,
}

/// TTF font loaded to GPU
#[derive(Clone)]
pub struct Font {
    pub(crate) font: Rc<fontdue::Font>,
    pub(crate) atlas: Rc<RefCell<Atlas>>,
    pub(crate) characters: Rc<RefCell<HashMap<(char, u16), CharacterInfo>>>,
}

#[derive(Debug, Default, Clone)]
pub struct TextDimensionsEx {
    pub width: f32,
    pub height: f32,
    pub offset_y: f32,
    pub chars: Vec<(Rect,Rect,)>,
    pub smooth: f32,
}

/// World space dimensions of the text, measured by "measure_text" function
#[derive(Debug, Default, Clone, Copy)]
pub struct TextDimensions {
    /// Distance from very left to very right of the rasterized text
    pub width: f32,
    /// Distance from the bottom to the top of the text.
    pub height: f32,
    /// Height offset from the baseline of the text.
    /// "draw_text(.., X, Y, ..)" will be rendered in a "Rect::new(X, Y - dimensions.offset_y, dimensions.width, dimensions.height)"
    /// For reference check "text_measures" example.
    pub offset_y: f32,
}

impl std::fmt::Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Font")
            .field("font", &"fontdue::Font")
            .finish()
    }
}

impl Font {
    pub(crate) fn load_from_bytes(atlas: Rc<RefCell<Atlas>>, bytes: &[u8]) -> Result<Font, Error> {
        Ok(Font {
            font: Rc::new(fontdue::Font::from_bytes(
                bytes,
                fontdue::FontSettings::default(),
            )?),
            characters: Rc::new(RefCell::new(HashMap::new())),
            atlas,
        })
    }

    pub(crate) fn set_atlas(&mut self, atlas: Rc<RefCell<Atlas>>) {
        self.atlas = atlas;
    }

    pub(crate) fn set_characters(
        &mut self,
        characters: Rc<RefCell<HashMap<(char, u16), CharacterInfo>>>,
    ) {
        self.characters = characters;
    }

    pub(crate) fn ascent(&self, font_size: f32) -> f32 {
        self.font.horizontal_line_metrics(font_size).unwrap().ascent
    }

    pub(crate) fn descent(&self, font_size: f32) -> f32 {
        self.font
            .horizontal_line_metrics(font_size)
            .unwrap()
            .descent
    }
    
    
    pub(crate) fn cache_glyph_many(
        font: &fontdue::Font,
        atlas: &mut Atlas,
        characters: &mut HashMap<(char, u16), CharacterInfo>,
        chars: impl Iterator<Item = char>,
        size: f32
    ) {
        for character in chars {
            if characters.contains_key(&(character, size.round() as u16)) {
                continue;
            }
            
            let (metrics, bitmap) = font.rasterize(character, size);
            
            let (width, height) = (metrics.width as u16, metrics.height as u16);
            
            
            let sprite = atlas.new_unique_id();
            atlas.cache_sprite(
                sprite,
                Image {
                    bytes: bitmap
                        .iter()
                        .flat_map(|coverage| vec![255, 255, 255, *coverage])
                        .collect(),
                    width,
                    height,
                },
            );
            let advance = metrics.advance_width;
            
            let (offset_x, offset_y) = (metrics.xmin, metrics.ymin);
            
            let character_info = CharacterInfo {
                advance,
                offset_x,
                offset_y,
                sprite,
            };
            
            characters
                .insert((character, size.round() as u16), character_info);
        }
    }
    
    pub(crate) fn cache_glyph(&self, char: char, size: f32) {
        Self::cache_glyph_many(
            self.font.deref(),
            self.atlas.borrow_mut().deref_mut(),
            self.characters.borrow_mut().deref_mut(),
            once(char),
            size
        );
    }

    pub(crate) fn measure_text(
        &self,
        text: impl AsRef<str>,
        font_size: f32,
        font_scale_x: f32,
        font_scale_y: f32,
        mut glyph_callback: impl FnMut(f32),
    ) -> TextDimensions {
        let text = text.as_ref();

        let dpi_scaling = miniquad::window::dpi_scale();
        let font_size = font_size * dpi_scaling;

        let mut width = 0.0;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for character in text.chars() {
            self.cache_glyph(character, font_size);

            let font_data = &self.characters.borrow_mut()[&(character, font_size.round() as u16)];
            let offset_y = font_data.offset_y as f32 * font_scale_y;

            let atlas = self.atlas.borrow_mut();
            let glyph = atlas.get(font_data.sprite).unwrap().rect;
            let advance = font_data.advance * font_scale_x;
            glyph_callback(advance);
            width += advance;
            min_y = min_y.min(offset_y);
            max_y = max_y.max(glyph.h * font_scale_y + offset_y);
        }

        TextDimensions {
            width: width / dpi_scaling,
            height: (max_y - min_y) / dpi_scaling,
            offset_y: max_y / dpi_scaling,
        }
    }
    
    /// 把自己包装成 `Rc<RefCell<Font>>` 便于使用
    pub fn shared(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

impl Font {
    /// List of ascii characters, may be helpful in combination with "populate_font_cache"
    pub fn ascii_character_list() -> Vec<char> {
        (0..255).filter_map(std::char::from_u32).collect()
    }

    /// List of latin characters
    pub fn latin_character_list() -> Vec<char> {
        "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#$%^&*(){}[].,:"
            .chars()
            .collect()
    }

    pub fn populate_font_cache(&self, characters: &[char], size: f32) {
        for character in characters {
            self.cache_glyph(*character, size);
        }
    }

    /// Sets the [FilterMode](https://docs.rs/miniquad/latest/miniquad/graphics/enum.FilterMode.html#) of this font's texture atlas.
    ///
    /// Use Nearest if you need integer-ratio scaling for pixel art, for example.
    ///
    /// # Example
    /// ```
    /// # use macroquad::prelude::*;
    /// # #[macroquad::main("test")]
    /// # async fn main() {
    /// let mut font = get_default_font();
    /// font.set_filter(FilterMode::Linear);
    /// # }
    /// ```
    pub fn set_filter(&mut self, filter_mode: miniquad::FilterMode) {
        self.atlas.borrow_mut().set_filter(filter_mode);
    }

    // pub fn texture(&self) -> Texture2D {
    //     let font = get_context().fonts_storage.get_font(*self);

    //     font.font_texture
    // }
}

/// Arguments for "draw_text_ex" function such as font, font_size etc
#[derive(Debug, Clone)]
pub struct TextParams {
    pub font: Option<Rc<RefCell<Font>>>,
    /// Base size for character height. The size in pixel used during font rasterizing.
    pub font_size: f32,
    /// The glyphs sizes actually drawn on the screen will be font_size * font_scale
    /// However with font_scale too different from 1.0 letters may be blurry
    pub font_scale: f32,
    /// Font X axis would be scaled by font_scale * font_scale_aspect
    /// and Y axis would be scaled by font_scale
    /// Default is 1.0
    pub font_scale_aspect: f32,
    /// Text rotation in radian
    /// Default is 0.0
    pub rotation: f32,
    pub color: Color,
}

impl<'a> Default for TextParams {
    fn default() -> TextParams {
        TextParams {
            font: None,
            font_size: 20.,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE,
            rotation: 0.0,
        }
    }
}

/// Load font from file with "path"
pub async fn load_ttf_font(path: &str) -> Result<Font, Error> {
    let bytes = crate::file::load_file(path)
        .await
        .map_err(|_| Error::FontError("The Font file couldn't be loaded"))?;

    load_ttf_font_from_bytes(&bytes[..])
}

/// Load font from bytes array, may be used in combination with include_bytes!
/// ```ignore
/// let font = load_ttf_font_from_bytes(include_bytes!("font.ttf"));
/// ```
pub fn load_ttf_font_from_bytes(bytes: &[u8]) -> Result<Font, Error> {
    let atlas = Rc::new(RefCell::new(Atlas::new(
        get_quad_context(),
        miniquad::FilterMode::Linear,
    )));

    let mut font = Font::load_from_bytes(atlas.clone(), bytes)?;

    font.populate_font_cache(&Font::ascii_character_list(), 15.);

    let ctx = get_context();

    font.set_filter(ctx.default_filter_mode);

    Ok(font)
}

/// Draw text with given font_size
/// Returns text size
pub fn draw_text(
    text: impl AsRef<str>,
    pos: impl Into<(f32, f32)>,
    font_size: f32,
    color: Color,
) -> TextDimensions {
    draw_text_ex(
        text,
        pos,
        TextParams {
            font_size,
            font_scale: 1.0,
            color,
            ..Default::default()
        },
    )
}

pub(crate) fn measure_text_ex_in(
    text: impl AsRef<str>,
    font: &fontdue::Font,
    atlas: &mut Atlas,
    characters: &mut HashMap<(char, u16), CharacterInfo>,
    rot: f32,
    font_size: f32,
    font_scale: f32,
    font_scale_aspect: f32,
) -> TextDimensionsEx {
    
    let text = text.as_ref();
    
    if text.is_empty() {
        return TextDimensionsEx::default();
    }
    
    
    let dpi_scaling = miniquad::window::dpi_scale();
    
    let rot_cos = rot.cos();
    let rot_sin = rot.sin();
    
    let font_scale_x = font_scale * font_scale_aspect;
    let font_scale_y = font_scale;
    let font_size = font_size * dpi_scaling;
    
    let mut total_width = 0.0;
    let mut max_offset_y = f32::MIN;
    let mut min_offset_y = f32::MAX;
    
    let mut data: Vec<(Rect, Rect,)> = Vec::with_capacity(text.len());
    
    Font::cache_glyph_many(
        font,
        atlas,
        characters,
        text.chars(), font_size
    );
    
    let raster_size = font_size.round() as u16;
    let smooth_scale = font_size / raster_size as f32;
    
    for character in text.chars() {
        let char_data = &characters[&(character, raster_size)];
        let offset_x = char_data.offset_x as f32 * font_scale_x * smooth_scale;
        let offset_y = char_data.offset_y as f32 * font_scale_y * smooth_scale;
        
        let glyph = atlas.get(char_data.sprite).unwrap().rect;
        let glyph_scaled_h = glyph.h * font_scale_y;
        
        min_offset_y = min_offset_y.min(offset_y);
        max_offset_y = max_offset_y.max(glyph_scaled_h + offset_y);
        
        let dest_x = (offset_x + total_width) * rot_cos + (glyph_scaled_h + offset_y) * rot_sin;
        let dest_y = (offset_x + total_width) * rot_sin + (-glyph_scaled_h - offset_y) * rot_cos;
        
        let dest = Rect::new(
            dest_x / dpi_scaling,
            dest_y / dpi_scaling,
            glyph.w / dpi_scaling * font_scale_x * smooth_scale,
            glyph.h / dpi_scaling * font_scale_y * smooth_scale,
        );
        
        total_width += char_data.advance * font_scale_x * smooth_scale;
        
        data.push((glyph, dest))
    }
    
    let total_height = max_offset_y - min_offset_y;
    
    
    TextDimensionsEx {
        width: total_width,
        height: total_height,
        offset_y: max_offset_y,
        chars: data,
        smooth: smooth_scale,
    }
}

pub fn measure_text_ex(
    text: impl AsRef<str>,
    font: Option<Rc<RefCell<Font>>>,
    rot: f32,
    font_size: f32,
    font_scale: f32,
    font_scale_aspect: f32,
) -> TextDimensionsEx {
    
    let text = text.as_ref();
    
    if text.is_empty() {
        return TextDimensionsEx::default();
    }
    
    let font_arc = font.map_or_else(
        || { get_default_font().clone() },
        |f| f,
    );
    let font_lock = font_arc.borrow();
    let font = font_lock.deref();
    
    let mut atlas_guard = font.atlas.borrow_mut();
    let mut chars_guard = font.characters.borrow_mut();
    let (font, atlas, characters) = (font.font.deref(), atlas_guard.deref_mut(), chars_guard.deref_mut());
    
    measure_text_ex_in(text,font,atlas,characters,rot,font_size,font_scale,font_scale_aspect)
}

/// Draw text with custom params such as font, font size and font scale
/// Returns text size
///
/// 其中 `center` 决定中心在贴图的何位置。为相对坐标。左下角为 (-1.0,-1.0)
pub fn draw_text_ex(
    text: impl AsRef<str>,
    pos: impl Into<(f32, f32)>,
    params: TextParams
) -> TextDimensions {
    let text = text.as_ref();
    
    if text.is_empty() {
        return TextDimensions::default();
    }
    
    let font_arc = params.font.map_or_else(
        || { get_default_font().clone() },
        |f| f,
    );
    let font_lock = font_arc.borrow();
    let font = font_lock.deref();
    
    
    let mut atlas_guard = font.atlas.borrow_mut();
    let mut chars_guard = font.characters.borrow_mut();
    let (font, atlas, characters) = (font.font.deref(), atlas_guard.deref_mut(), chars_guard.deref_mut());
    
    draw_text_ex_in(
        text,
        font,
        atlas,
        characters,
        params.rotation,
        params.font_size,
        params.font_scale,
        params.font_scale_aspect,
        params.color,
        pos,
        (-1.,-1.)
    )
}


pub(crate) fn draw_text_ex_in(
    text: impl AsRef<str>,
    font: &fontdue::Font,
    atlas: &mut Atlas,
    characters: &mut HashMap<(char, u16), CharacterInfo>,
    rot: f32,
    font_size: f32,
    font_scale: f32,
    font_scale_aspect: f32,
    color: Color,
    pos: impl Into<(f32, f32)>,
    center: impl Into<(f32, f32)>,
) -> TextDimensions {
    let (center_x, center_y) = center.into(); // 提取center相对坐标
    
    let rot_cos = rot.cos();
    let rot_sin = rot.sin();
    
    let dim =
        measure_text_ex_in(
            text,
            font, atlas, characters,
            rot,
            font_size,
            font_scale,
            font_scale_aspect,
        );
    
    let dpi_scaling = miniquad::window::dpi_scale();
    
    let total_width = dim.width;
    let total_height = dim.height;
    let oy = dim.offset_y;
    if total_width < 1e-6 || total_height < 1e-6 {
        return TextDimensions {
            width: 0.,
            height: 0.,
            offset_y: oy / dpi_scaling,
        }
    }
    let (ox, oy) = pos.into();
    // dx = x×|w|×cosθ - y×|h|×sinθ
    // dy = x×|w|×sinθ + y×|h|×cosθ
    let cdx = (center_x+1.)/2.;
    let cdy = (center_y+1.)/2.;
    let fx = cdx * dim.width;
    let fy = -cdy * dim.height;
    let dx = fx * rot_cos - fy * rot_sin;
    let dy = fx * rot_sin + fy * rot_cos;
    
    let (x, y) = (ox - dx, oy - dy);
    let data = dim.chars;
    
    for (glyph, dest) in data {
        let new_center = vec2(x + dest.x,  y + dest.y);
        crate::texture::draw_texture_ex(
            &crate::texture::Texture2D {
                texture: TextureHandle::Unmanaged(atlas.texture()),
            },
            new_center,
            color,
            crate::texture::DrawTextureParams {
                dest_size: Some(vec2(dest.w, dest.h)),
                source: Some(glyph),
                rotation: rot,
                pivot: Some(new_center),
                ..Default::default()
            },
        );
    }
    
    TextDimensions {
        width: total_width / dpi_scaling,
        height: total_height / dpi_scaling,
        offset_y: oy / dpi_scaling,
    }
}

/// Draw multiline text with the given font_size, line_distance_factor and color.
/// If no line distance but a custom font is given, the fonts line gap will be used as line distance factor if it exists.
pub fn draw_multiline_text(
    text: impl AsRef<str>,
    pos: impl Into<(f32,f32)>,
    font_size: f32,
    line_distance_factor: Option<f32>,
    color: Color,
) -> TextDimensions {
    draw_multiline_text_ex(
        text,
        pos,
        line_distance_factor,
        TextParams {
            font_size,
            font_scale: 1.0,
            color,
            ..Default::default()
        },
    )
}

/// Draw multiline text with the given line distance and custom params such as font, font size and font scale.
/// If no line distance but a custom font is given, the fonts newline size will be used as line distance factor if it exists, else default to font size.
pub fn draw_multiline_text_ex(
    text: impl AsRef<str>,
    pos: impl Into<(f32,f32)>,
    line_distance_factor: Option<f32>,
    params: TextParams,
) -> TextDimensions {
    let text = text.as_ref();
    
    if text.is_empty() {
        return TextDimensions::default();
    }
    
    let (mut x, mut y) = pos.into(); // 原始起点
    
    let font_arc = if let Some(font) = params.font {
        font
    } else {
        get_default_font()
    };
    let font_lock = font_arc.borrow();
    let font = font_lock.deref();
    
    let line_distance = match line_distance_factor {
        Some(distance) => distance,
        None => {
            let mut font_line_distance = 0.0;
            
            if let Some(metrics) = font.font.horizontal_line_metrics(1.0) {
                font_line_distance = metrics.new_line_size;
            }

            font_line_distance
        }
    };

    let mut dimensions = TextDimensions::default();
    let y_step = line_distance * params.font_size  * params.font_scale;

    let mut positions: Vec<(f32,f32)> = vec![];
    
    let dx = (line_distance * params.font_size * params.font_scale) * params.rotation.sin();
    let dy = (line_distance * params.font_size * params.font_scale) * params.rotation.cos();
    
    let mut atlas_guard = font.atlas.borrow_mut();
    let mut chars_guard = font.characters.borrow_mut();
    let (font, atlas, characters) = (font.font.deref(), atlas_guard.deref_mut(), chars_guard.deref_mut());
    
    for line in text.lines() {
        positions.push((x,y));
        x -= dx;
        y += dy;
        
        let line_dimensions = measure_text_ex_in(line, font, atlas, characters, params.rotation, params.font_size, params.font_scale, params.font_scale_aspect);
        
        dimensions.width = f32::max(dimensions.width, line_dimensions.width);
        dimensions.height += y_step;

        if dimensions.offset_y == 0.0 {
            dimensions.offset_y = line_dimensions.offset_y;
        }
    }
    
    for (idx,line) in text.lines().enumerate() {
        draw_text_ex_in(
            line,
            font, atlas, characters,
            params.rotation,
            params.font_size,
            params.font_scale,
            params.font_scale_aspect,
            params.color,
            positions[idx], (-1., -1.)
        );
    }

    dimensions
}

/// Get the text center.
pub fn get_text_center(
    text: impl AsRef<str>,
    font: Option<Rc<RefCell<Font>>>,
    font_size: f32,
    font_scale: f32,
    rotation: f32,
) -> crate::Vec2 {
    let measure = measure_text(text, font, font_size, font_scale);

    let x_center = measure.width / 2.0 * rotation.cos() + measure.height / 2.0 * rotation.sin();
    let y_center = measure.width / 2.0 * rotation.sin() - measure.height / 2.0 * rotation.cos();

    crate::Vec2::new(x_center, y_center)
}

pub fn measure_text(
    text: impl AsRef<str>,
    font: Option<Rc<RefCell<Font>>>,
    font_size: f32,
    font_scale: f32,
) -> TextDimensions {
    let font_arc = font.unwrap_or_else(|| get_default_font().clone());
    let font_lock = font_arc.borrow();
    let font = font_lock.deref();
    font.measure_text(text, font_size, font_scale, font_scale, |_| {})
}

pub fn measure_multiline_text(
    text: &str,
    font: Option<Rc<RefCell<Font>>>,
    font_size: f32,
    font_scale: f32,
    line_distance_factor: Option<f32>,
) -> TextDimensions {
    let font_arc = font.unwrap_or_else(|| get_default_font().clone());
    let font_lock = font_arc.borrow();
    let font = font_lock.deref();
    let line_distance = match line_distance_factor {
        Some(distance) => distance,
        None => match font.font.horizontal_line_metrics(1.0) {
            Some(metrics) => metrics.new_line_size,
            None => 1.0,
        },
    };

    let mut dimensions = TextDimensions::default();
    let y_step = line_distance * font_size * font_scale;

    for line in text.lines() {
        let line_dimensions = font.measure_text(line, font_size, font_scale, font_scale, |_| {});

        dimensions.width = f32::max(dimensions.width, line_dimensions.width);
        dimensions.height += y_step;
        if dimensions.offset_y == 0.0 {
            dimensions.offset_y = line_dimensions.offset_y;
        }
    }

    dimensions
}

/// Converts word breaks to newlines wherever the text would otherwise exceed the given length.
pub fn wrap_text(
    text: &str,
    font: Option<Rc<RefCell<Font>>>,
    font_size: f32,
    font_scale: f32,
    maximum_line_length: f32,
) -> String {
    let font_rc = font.unwrap_or_else(|| get_default_font().clone());
    let mut font_ref = font_rc.borrow_mut();
    let font = font_ref.deref_mut();

    // This is always a bit too much memory, but it saves a lot of reallocations.
    let mut new_text =
        String::with_capacity(text.len() + text.chars().filter(|c| c.is_whitespace()).count());

    let mut current_word_start = 0usize;
    let mut current_word_end = 0usize;
    let mut characters = text.char_indices();
    let mut total_width = 0.0;
    let mut word_width = 0.0;

    font.measure_text(text, font_size, font_scale, font_scale, |mut width| {
        // It's impossible this is called more often than the text has characters.
        let (idx, c) = characters.next().unwrap();
        let mut keep_char = true;

        if c.is_whitespace() {
            new_text.push_str(&text[current_word_start..idx + c.len_utf8()]);
            current_word_start = idx + c.len_utf8();
            word_width = 0.0;
            keep_char = false;

            // If we would wrap, ignore the whitespace.
            if total_width + width > maximum_line_length {
                width = 0.0;
            }
        }

        // If a single word expands past the length limit, just break it up.
        if word_width + width > maximum_line_length {
            new_text.push_str(&text[current_word_start..current_word_end]);
            new_text.push('\n');
            current_word_start = current_word_end;
            total_width = 0.0;
            word_width = 0.0;
        }

        current_word_end = idx + c.len_utf8();
        if keep_char {
            word_width += width;
        }

        if c == '\n' {
            total_width = 0.0;
            word_width = 0.0;
            return;
        }

        total_width += width;

        if total_width > maximum_line_length {
            new_text.push('\n');
            total_width = word_width;
        }
    });

    new_text.push_str(&text[current_word_start..current_word_end]);

    new_text
}

pub(crate) struct FontsStorage {
    pub default_font: Rc<RefCell<Font>>,
}

impl FontsStorage {
    pub(crate) fn new(ctx: &mut dyn miniquad::RenderingBackend) -> FontsStorage {
        let atlas = Rc::new(RefCell::new(Atlas::new(ctx, miniquad::FilterMode::Linear)));

        let default_font = Rc::new(RefCell::new(Font::load_from_bytes(atlas, include_bytes!("ProggyClean.ttf")).unwrap()));
        FontsStorage { default_font }
    }
    
    
    pub fn get_default_font(&self) -> Rc<RefCell<Font>> {
        self.default_font.clone()
    }
    
    pub fn set_default_font(&mut self, font: Rc<RefCell<Font>>) {
        self.default_font = font;
    }
}

/// Returns macroquads default font.
pub fn get_default_font() -> Rc<RefCell<Font>> {
    let context = get_context();
    context.fonts_storage.default_font.clone()
}

/// Replaces macroquads default font with `font`.
pub fn set_default_font(font: Font) {
    let context = get_context();
    context.fonts_storage.default_font = Rc::new(RefCell::new((font)));
}

/// From given font size in world space gives
/// (font_size, font_scale and font_aspect) params to make rasterized font
/// looks good in currently active camera
pub fn camera_font_scale(world_font_size: f32) -> (f32, f32, f32) {
    let context = get_context();
    let (scr_w, scr_h) = miniquad::window::screen_size();
    let cam_space = context
        .projection_matrix()
        .inverse()
        .transform_vector3(vec3(2., 2., 0.));
    let (cam_w, cam_h) = (cam_space.x.abs(), cam_space.y.abs());

    let screen_font_size = world_font_size * scr_h / cam_h;

    let font_size = screen_font_size;

    (font_size, cam_h / scr_h, scr_h / scr_w * cam_w / cam_h)
}
