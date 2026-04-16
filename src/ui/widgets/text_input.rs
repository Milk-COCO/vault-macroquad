//! This module defines the [`TextInput`] widget that allows the user to enter text.
use miniquad::window;
use super::{Action, Align, Button, Container, Direction, Widget};
use crate::prelude::*;
use arboard::Clipboard;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use crate::get_context;

/// 文本选择范围
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Selection {
    start: u32,
    end: u32,
}

impl Selection {
    fn new(pos: u32) -> Self {
        Self { start: pos, end: pos }
    }
    
    fn is_empty(&self) -> bool {
        self.start == self.end
    }
    
    fn min(&self) -> u32 {
        self.start.min(self.end)
    }
    
    fn max(&self) -> u32 {
        self.start.max(self.end)
    }
}

/// 键盘重复输入状态
struct KeyRepeatState {
    pressed: bool,
    first_press_time: Option<Instant>,
    last_repeat_time: Option<Instant>,
}

impl KeyRepeatState {
    fn new() -> Self {
        Self {
            pressed: false,
            first_press_time: None,
            last_repeat_time: None,
        }
    }
    
    fn update(&mut self, is_down: bool) -> bool {
        let now = Instant::now();
        
        if is_down {
            if !self.pressed {
                self.pressed = true;
                self.first_press_time = Some(now);
                self.last_repeat_time = None;
                true
            } else {
                let first = self.first_press_time.unwrap();
                let initial_delay = 0.5;
                let repeat_interval = 0.05;
                
                if now.duration_since(first).as_secs_f32() > initial_delay {
                    if let Some(last) = self.last_repeat_time {
                        if now.duration_since(last).as_secs_f32() > repeat_interval {
                            self.last_repeat_time = Some(now);
                            true
                        } else {
                            false
                        }
                    } else {
                        self.last_repeat_time = Some(now);
                        true
                    }
                } else {
                    false
                }
            }
        } else {
            self.pressed = false;
            self.first_press_time = None;
            self.last_repeat_time = None;
            false
        }
    }
}

/// The [`TextInput`] widget that allows the user to enter text.
pub struct TextInput {
    text: String,
    cursor_pos: u32,
    selection: Option<Selection>,
    
    text_color: Color,
    hovered_text_color: Color,
    bg: Color,
    fg: Color,
    placeholder: Option<String>,
    placeholder_color: Color,
    
    hover: bool,
    just_clicked: bool,
    selected: bool,
    
    font: Option<Rc<RefCell<Font>>>,
    width: f32,
    height: f32,
    
    cursor_blink_timer: f32,
    cursor_visible: bool,
    
    is_password: bool,
    max_length: Option<usize>,
    
    is_dragging: bool,
    just_submitted: bool,
    
    last_click_time: Option<Instant>,
    last_click_pos: (f32, f32),
    
    key_left: KeyRepeatState,
    key_right: KeyRepeatState,
    key_backspace: KeyRepeatState,
    key_delete: KeyRepeatState,
    
    long_press_start: Option<Instant>,
    long_press_initial_pos: (f32, f32),
    context_menu_open: bool,
    context_menu_pos: (f32, f32),
    context_menu_container: Container,
}

impl TextInput {
    pub fn new(max_length: Option<usize>, width: f32, height: f32, text_color: Color, hovered_text_color: Color, bg: Color, fg: Color, font: Option<Rc<RefCell<Font>>>) -> Self {
        let mut menu_container = Container::new(
            Direction::Vertical,
            Align::Start,
            0.0,
            fg,
            Some((1.0, 1.0, 1.0, 1.0)),
            None
        );
        menu_container.add_child(Button::new(75.0, 28.0, "Cut".to_string(), text_color, hovered_text_color, bg, fg, font.clone()));
        menu_container.add_child(Button::new(75.0, 28.0, "Copy".to_string(), text_color, hovered_text_color, bg, fg, font.clone()));
        menu_container.add_child(Button::new(75.0, 28.0, "Paste".to_string(), text_color, hovered_text_color, bg, fg, font.clone()));
        
        Self {
            text: String::new(),
            cursor_pos: 0,
            selection: None,
            
            text_color,
            hovered_text_color,
            bg,
            fg,
            placeholder: None,
            placeholder_color: Color::new(0.5, 0.5, 0.5, 1.0),
            
            hover: false,
            just_clicked: false,
            selected: false,
            
            font,
            width,
            height,
            
            cursor_blink_timer: 0.0,
            cursor_visible: true,
            
            is_password: false,
            max_length,
            
            is_dragging: false,
            just_submitted: false,
            
            last_click_time: None,
            last_click_pos: (0.0, 0.0),
            
            key_left: KeyRepeatState::new(),
            key_right: KeyRepeatState::new(),
            key_backspace: KeyRepeatState::new(),
            key_delete: KeyRepeatState::new(),
            
            long_press_start: None,
            long_press_initial_pos: (0.0, 0.0),
            context_menu_open: false,
            context_menu_pos: (0.0, 0.0),
            context_menu_container: menu_container,
        }
    }
    
    pub fn with_placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    
    pub fn with_password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }
    
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max.max(1));
        if let Some(max) = self.max_length {
            let chars: Vec<char> = self.text.chars().take(max).collect();
            self.text = chars.into_iter().collect();
            self.cursor_pos = self.cursor_pos.min(self.text.chars().count() as u32);
        }
        self
    }
    
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    pub fn set_text(&mut self, text: impl Into<String>) {
        let new_text = text.into();
        self.text = new_text;
        
        let count = self.text.chars().count() as u32;
        self.cursor_pos = self.cursor_pos.min(count);
        self.selection = None;
    }
    
    pub fn clear(&mut self) {
        self.set_text("");
    }
    
    pub fn is_submitted(&self) -> bool {
        self.just_submitted
    }
    
    fn char_idx_to_byte_idx(&self, s: &str, char_idx: usize) -> usize {
        s.chars().take(char_idx).map(|c| c.len_utf8()).sum()
    }
    
    fn display_text(&self) -> String {
        if self.is_password {
            "*".repeat(self.text.chars().count())
        } else {
            self.text.clone()
        }
    }
    
    fn find_word_boundary(&self, mut pos: u32, forward: bool) -> u32 {
        let chars: Vec<char> = self.text.chars().collect();
        if forward {
            while pos < chars.len() as u32 && chars[pos as usize].is_alphanumeric() {
                pos += 1;
            }
            while pos < chars.len() as u32 && !chars[pos as usize].is_alphanumeric() {
                pos += 1;
            }
        } else {
            while pos > 0 && !chars[(pos - 1) as usize].is_alphanumeric() {
                pos -= 1;
            }
            while pos > 0 && chars[(pos - 1) as usize].is_alphanumeric() {
                pos -= 1;
            }
        }
        pos
    }
    
    fn measure_ex(&self, text: &str) -> TextDimensionsEx {
        measure_text_ex(
            text,
            self.font.clone(),
            0.0,
            self.height * 0.4,
            1.0,
            1.0,
        )
    }
    
    fn measure_prefix_width(&self, chars: impl Iterator<Item = char>) -> f32 {
        let s: String = chars.collect();
        self.measure_ex(&s).width
    }
    
    fn text_width(&self, text: &str) -> f32 {
        self.measure_ex(text).width
    }
    
    fn check_double_click(&mut self, mouse_pos: (f32, f32)) -> bool {
        let now = Instant::now();
        let (mx, my) = mouse_pos;
        
        if let Some(last_time) = self.last_click_time {
            let (last_x, last_y) = self.last_click_pos;
            let time_diff = now.duration_since(last_time).as_millis();
            let pos_diff = ((mx - last_x).powi(2) + (my - last_y).powi(2)).sqrt();
            
            if time_diff < 300 && pos_diff < 5.0 {
                self.last_click_time = None;
                return true;
            }
        }
        
        self.last_click_time = Some(now);
        self.last_click_pos = mouse_pos;
        false
    }
    
    fn delete_selection(&mut self) {
        if let Some(sel) = self.selection {
            if !sel.is_empty() {
                let start_char = sel.min() as usize;
                let end_char = sel.max() as usize;
                let start_byte = self.char_idx_to_byte_idx(&self.text, start_char);
                let end_byte = self.char_idx_to_byte_idx(&self.text, end_char);
                
                let mut new_text = String::with_capacity(self.text.len());
                new_text.push_str(&self.text[..start_byte]);
                new_text.push_str(&self.text[end_byte..]);
                self.set_text(new_text);
                self.cursor_pos = start_char as u32;
            }
        }
    }
    
    fn insert(&mut self, text: &str) {
        self.delete_selection();
        let len = text.len();
        if len == 0 { return; }
        let old_cursor_char = self.cursor_pos as usize;
        let old_cursor_byte = self.char_idx_to_byte_idx(&self.text, old_cursor_char);
        
        let mut new_text = String::with_capacity(self.text.len() + len);
        new_text.push_str(&self.text[..old_cursor_byte]);
        new_text.push_str(text);
        new_text.push_str(&self.text[old_cursor_byte..]);
        
        self.set_text(new_text);
        self.cursor_pos = (old_cursor_char + text.chars().count()) as u32;
        self.cursor_visible = true;
        self.cursor_blink_timer = 0.0;
    }
    
    fn calculate_clamped_menu_pos(&self, initial_pos: (f32, f32)) -> (f32, f32) {
        let menu_width = self.context_menu_container.width();
        let menu_height = self.context_menu_container.height();
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        let mut x = initial_pos.0;
        let mut y = initial_pos.1;
        
        if x + menu_width > screen_w {
            x = screen_w - menu_width;
        }
        if x < 0.0 {
            x = 0.0;
        }
        if y + menu_height > screen_h {
            y = screen_h - menu_height;
        }
        if y < 0.0 {
            y = 0.0;
        }
        
        (x, y)
    }
    
    pub fn draw_context_menu(&self) {
        if self.context_menu_open {
            self.context_menu_container.draw(self.context_menu_pos);
        }
    }
}

impl Widget for TextInput {
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
        (|| {
            let (x, y) = pos.into();
            let mouse_pos = mouse_position();
            let mx = mouse_pos.0;
            let my = mouse_pos.1;
            
            let padding = 4.0;
            
            self.just_clicked = false;
            self.just_submitted = false;
            
            self.hover = mx >= x && mx <= x + self.width && my >= y && my <= y + self.height;
            
            let clicked = is_mouse_button_pressed(MouseButton::Left);
            let mouse_down = is_mouse_button_down(MouseButton::Left);
            
            if self.context_menu_open {
                self.context_menu_container.process(self.context_menu_pos);
                
                if let Some(cut_btn) = self.context_menu_container.get_child_as::<Button>(0) {
                    if cut_btn.is_clicked() {
                        if let Some(sel) = self.selection {
                            if !sel.is_empty() {
                                let start_char = sel.min() as usize;
                                let end_char = sel.max() as usize;
                                let start_byte = self.char_idx_to_byte_idx(&self.text, start_char);
                                let end_byte = self.char_idx_to_byte_idx(&self.text, end_char);
                                let selected_text = &self.text[start_byte..end_byte];
                                if let Ok(mut clipboard) = Clipboard::new() {
                                    clipboard.set_text(selected_text).ok();
                                }
                                let mut new_text = String::new();
                                new_text.push_str(&self.text[..start_byte]);
                                new_text.push_str(&self.text[end_byte..]);
                                self.set_text(new_text);
                                self.cursor_pos = start_char as u32;
                            }
                        }
                        self.context_menu_open = false;
                        self.long_press_start = None;
                        self.is_dragging = false;
                    }
                }
                
                if let Some(copy_btn) = self.context_menu_container.get_child_as::<Button>(1) {
                    if copy_btn.is_clicked() {
                        if let Some(sel) = self.selection {
                            if !sel.is_empty() {
                                let start_char = sel.min() as usize;
                                let end_char = sel.max() as usize;
                                let start_byte = self.char_idx_to_byte_idx(&self.text, start_char);
                                let end_byte = self.char_idx_to_byte_idx(&self.text, end_char);
                                let selected_text = &self.text[start_byte..end_byte];
                                if let Ok(mut clipboard) = Clipboard::new() {
                                    clipboard.set_text(selected_text).ok();
                                }
                            }
                        }
                        self.context_menu_open = false;
                        self.long_press_start = None;
                        self.is_dragging = false;
                    }
                }
                
                if let Some(paste_btn) = self.context_menu_container.get_child_as::<Button>(2) {
                    if paste_btn.is_clicked() {
                        if let Ok(mut clipboard) = Clipboard::new() {
                            if let Ok(paste_text) = clipboard.get_text() {
                                self.delete_selection();
                                let old_cursor_char = self.cursor_pos as usize;
                                let old_cursor_byte = self.char_idx_to_byte_idx(&self.text, old_cursor_char);
                                let mut new_text = String::new();
                                new_text.push_str(&self.text[..old_cursor_byte]);
                                new_text.push_str(&paste_text);
                                new_text.push_str(&self.text[old_cursor_byte..]);
                                self.set_text(new_text);
                                self.cursor_pos = (old_cursor_char + paste_text.chars().count()) as u32;
                            }
                        }
                        self.context_menu_open = false;
                        self.long_press_start = None;
                        self.is_dragging = false;
                    }
                }
                
                if clicked {
                    let menu_rect = Rect::new(
                        self.context_menu_pos.0,
                        self.context_menu_pos.1,
                        self.context_menu_container.width(),
                        self.context_menu_container.height(),
                    );
                    if !menu_rect.contains(mouse_pos.into()) {
                        self.context_menu_open = false;
                        self.long_press_start = None;
                        self.is_dragging = false;
                    }
                }
                return;
            }
            
            let right_clicked = is_mouse_button_pressed(MouseButton::Right);
            if right_clicked && self.hover {
                self.context_menu_open = true;
                self.context_menu_pos = self.calculate_clamped_menu_pos(mouse_pos);
                return;
            }
            
            if clicked && self.hover {
                self.long_press_start = Some(Instant::now());
                self.long_press_initial_pos = mouse_pos;
                self.selected = true;
                self.just_clicked = true;
            }
            
            let threshold = screen_width().min(screen_height()) * 0.01;
            let mut cancel_long_press = false;
            
            if mouse_down && self.long_press_start.is_some() {
                let (ix, iy) = self.long_press_initial_pos;
                let dist = ((mx - ix).powi(2) + (my - iy).powi(2)).sqrt();
                if dist > threshold {
                    cancel_long_press = true;
                }
            }
            
            if cancel_long_press {
                self.long_press_start = None;
                self.is_dragging = true;
                
                let display_text = self.display_text();
                let avail = self.width - padding * 2.0;
                let total_w = self.text_width(&display_text);
                
                let mut scroll = 0.0;
                if total_w > avail {
                    let cursor_w = self.measure_prefix_width(display_text.chars().take(self.cursor_pos as usize));
                    scroll = (cursor_w - avail / 2.0).max(0.0).min(total_w - avail);
                }
                
                let target = mx - x - padding + scroll;
                let mut accum = 0.0;
                let mut new_pos = 0;
                for (i, c) in display_text.chars().enumerate() {
                    let w = self.measure_prefix_width(std::iter::once(c));
                    if target < accum + w * 0.5 {
                        break;
                    }
                    accum += w;
                    new_pos = i + 1;
                }
                
                self.cursor_pos = new_pos as u32;
                self.selection = Some(Selection::new(self.cursor_pos));
            }
            
            let mut long_trigger = false;
            if mouse_down && self.long_press_start.is_some() {
                let t = self.long_press_start.unwrap().elapsed().as_millis();
                let (ix, iy) = self.long_press_initial_pos;
                let d = ((mx - ix).powi(2) + (my - iy).powi(2)).sqrt();
                if t > 450 && d < threshold {
                    self.context_menu_open = true;
                    self.context_menu_pos = self.calculate_clamped_menu_pos(mouse_pos);
                    self.long_press_start = None;
                    long_trigger = true;
                    return;
                }
            }
            
            if !mouse_down && self.long_press_start.is_some() && !long_trigger {
                self.is_dragging = true;
                let double = self.check_double_click(mouse_pos);
                
                let display_text = self.display_text();
                let avail = self.width - padding * 2.0;
                let total_w = self.text_width(&display_text);
                let mut scroll = 0.0;
                if total_w > avail {
                    let cursor_w = self.measure_prefix_width(display_text.chars().take(self.cursor_pos as usize));
                    scroll = (cursor_w - avail / 2.0).max(0.0).min(total_w - avail);
                }
                
                let target = mx - x - padding + scroll;
                let mut accum = 0.0;
                let mut new_pos = 0;
                for (i, c) in display_text.chars().enumerate() {
                    let w = self.measure_prefix_width(std::iter::once(c));
                    if target < accum + w * 0.5 {
                        break;
                    }
                    accum += w;
                    new_pos = i + 1;
                }
                
                self.cursor_pos = new_pos as u32;
                
                if double {
                    let s = self.find_word_boundary(self.cursor_pos, false);
                    let e = self.find_word_boundary(self.cursor_pos, true);
                    self.selection = Some(Selection { start: s, end: e });
                } else {
                    self.selection = Some(Selection::new(self.cursor_pos));
                }
                
                self.cursor_visible = true;
                self.cursor_blink_timer = 0.0;
                self.long_press_start = None;
            }
            
            if self.is_dragging && mouse_down {
                let display_text = self.display_text();
                let avail = self.width - padding * 2.0;
                let total_w = self.text_width(&display_text);
                let mut scroll = 0.0;
                if total_w > avail {
                    let cursor_w = self.measure_prefix_width(display_text.chars().take(self.cursor_pos as usize));
                    scroll = (cursor_w - avail / 2.0).max(0.0).min(total_w - avail);
                }
                
                let target = mx - x - padding + scroll;
                let mut accum = 0.0;
                let mut pos = 0;
                for (i, c) in display_text.chars().enumerate() {
                    let w = self.measure_prefix_width(std::iter::once(c));
                    if target < accum + w * 0.5 {
                        break;
                    }
                    accum += w;
                    pos = i + 1;
                }
                
                if let Some(ref mut sel) = self.selection {
                    sel.end = pos as u32;
                }
                self.cursor_pos = pos as u32;
            } else if !mouse_down {
                self.is_dragging = false;
            }
            
            if clicked && !self.hover {
                self.selected = false;
                self.is_dragging = false;
                self.selection = None;
                self.long_press_start = None;
                window::set_ime_enabled(false);
            }
            
            if self.selected && is_key_pressed(KeyCode::Escape) {
                self.selected = false;
                self.selection = None;
                window::set_ime_enabled(false);
            }
            
            if self.selected {
                window::set_ime_enabled(true);
                
                self.cursor_blink_timer += get_frame_time();
                if self.cursor_blink_timer > 0.5 {
                    self.cursor_blink_timer = 0.0;
                    self.cursor_visible = !self.cursor_visible;
                }
                
                let display_text = self.display_text();
                let available_width = self.width - 8.0;
                let total_width = self.text_width(&display_text);
                let mut scroll_offset = 0.0;
                if total_width > available_width {
                    let cursor_w = self.measure_prefix_width(display_text.chars().take(self.cursor_pos as usize));
                    scroll_offset = (cursor_w - available_width * 0.5).max(0.0).min(total_width - available_width);
                }
                
                let cursor_w = self.measure_prefix_width(display_text.chars().take(self.cursor_pos as usize));
                let (win_x, win_y) = window::get_window_position();
                let cx = win_x as f32 + x + 4.0 + cursor_w - scroll_offset;
                let cy = win_y as f32 + y + self.height * 0.5 - 8.0;
                window::set_ime_position(cx as i32, cy as i32);
                
                let mut input = String::new();
                if let Some(ref t) = get_context().ime_commit_string {
                    // info!("{}",t);
                    if !t.is_empty() { input.push_str(t); }
                } else {
                    let preedit = &get_context().ime_preedit_string;
                    if preedit.is_empty() {
                        let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
                        let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
                        
                        if ctrl && is_key_pressed(KeyCode::A) {
                            self.selection = Some(Selection { start: 0, end: self.text.chars().count() as u32 });
                            self.cursor_pos = self.text.chars().count() as u32;
                            self.cursor_visible = true;
                            self.cursor_blink_timer = 0.0;
                        }
                        if ctrl && is_key_pressed(KeyCode::X) {
                            if let Some(sel) = self.selection {
                                if !sel.is_empty() {
                                    let s = sel.min() as usize;
                                    let e = sel.max() as usize;
                                    let sb = self.char_idx_to_byte_idx(&self.text, s);
                                    let eb = self.char_idx_to_byte_idx(&self.text, e);
                                    let t = &self.text[sb..eb];
                                    if let Ok(mut cb) = Clipboard::new() { cb.set_text(t).ok(); }
                                    let mut nt = String::new();
                                    nt.push_str(&self.text[..sb]);
                                    nt.push_str(&self.text[eb..]);
                                    self.set_text(nt);
                                    self.cursor_pos = s as u32;
                                }
                            }
                        }
                        if ctrl && is_key_pressed(KeyCode::C) {
                            if let Some(sel) = self.selection {
                                if !sel.is_empty() {
                                    let s = sel.min() as usize;
                                    let e = sel.max() as usize;
                                    let sb = self.char_idx_to_byte_idx(&self.text, s);
                                    let eb = self.char_idx_to_byte_idx(&self.text, e);
                                    let t = &self.text[sb..eb];
                                    if let Ok(mut cb) = Clipboard::new() { cb.set_text(t).ok(); }
                                }
                            }
                        }
                        if ctrl && is_key_pressed(KeyCode::V) {
                            if let Ok(mut cb) = Clipboard::new() {
                                if let Ok(pt) = cb.get_text() {
                                    self.delete_selection();
                                    let oc = self.cursor_pos as usize;
                                    let ob = self.char_idx_to_byte_idx(&self.text, oc);
                                    let mut nt = String::new();
                                    nt.push_str(&self.text[..ob]);
                                    nt.push_str(&pt);
                                    nt.push_str(&self.text[ob..]);
                                    self.set_text(nt);
                                    self.cursor_pos = (oc + pt.chars().count()) as u32;
                                }
                            }
                        }
                        
                        if self.key_left.update(is_key_down(KeyCode::Left)) {
                            if ctrl {
                                self.cursor_pos = self.find_word_boundary(self.cursor_pos, false);
                            } else {
                                self.cursor_pos = self.cursor_pos.saturating_sub(1);
                            }
                            if !shift { self.selection = None; } else if let Some(ref mut s) = self.selection { s.end = self.cursor_pos; } else { self.selection = Some(Selection { start: self.cursor_pos + 1, end: self.cursor_pos }); }
                            self.cursor_visible = true;
                            self.cursor_blink_timer = 0.0;
                        }
                        if self.key_right.update(is_key_down(KeyCode::Right)) {
                            let cnt = self.text.chars().count() as u32;
                            if ctrl {
                                self.cursor_pos = self.find_word_boundary(self.cursor_pos, true);
                            } else {
                                self.cursor_pos = (self.cursor_pos + 1).min(cnt);
                            }
                            if !shift { self.selection = None; } else if let Some(ref mut s) = self.selection { s.end = self.cursor_pos; } else { self.selection = Some(Selection { start: self.cursor_pos - 1, end: self.cursor_pos }); }
                            self.cursor_visible = true;
                            self.cursor_blink_timer = 0.0;
                        }
                        
                        if is_key_pressed(KeyCode::Home) {
                            self.cursor_pos = 0;
                            if !shift { self.selection = None; } else if let Some(ref mut s) = self.selection { s.end = 0; }
                            self.cursor_visible = true;
                            self.cursor_blink_timer = 0.0;
                        }
                        if is_key_pressed(KeyCode::End) {
                            let cnt = self.text.chars().count() as u32;
                            self.cursor_pos = cnt;
                            if !shift { self.selection = None; } else if let Some(ref mut s) = self.selection { s.end = cnt; }
                            self.cursor_visible = true;
                            self.cursor_blink_timer = 0.0;
                        }
                        
                        if self.key_backspace.update(is_key_down(KeyCode::Backspace)) {
                            self.delete_selection();
                            let oc = self.cursor_pos as usize;
                            if oc > 0 {
                                let ob = self.char_idx_to_byte_idx(&self.text, oc);
                                let nb = self.char_idx_to_byte_idx(&self.text, oc - 1);
                                let mut nt = String::new();
                                nt.push_str(&self.text[..nb]);
                                nt.push_str(&self.text[ob..]);
                                self.set_text(nt);
                                self.cursor_pos = (oc - 1) as u32;
                            }
                            self.cursor_visible = true;
                            self.cursor_blink_timer = 0.0;
                        }
                        if self.key_delete.update(is_key_down(KeyCode::Delete)) {
                            self.delete_selection();
                            let oc = self.cursor_pos as usize;
                            let cnt = self.text.chars().count();
                            if oc < cnt {
                                let ob = self.char_idx_to_byte_idx(&self.text, oc);
                                let eb = self.char_idx_to_byte_idx(&self.text, oc + 1);
                                let mut nt = String::new();
                                nt.push_str(&self.text[..ob]);
                                nt.push_str(&self.text[eb..]);
                                self.set_text(nt);
                                self.cursor_pos = oc as u32;
                            }
                            self.cursor_visible = true;
                            self.cursor_blink_timer = 0.0;
                        }
                        
                        if is_key_pressed(KeyCode::Enter) {
                            self.just_submitted = true;
                        }
                        
                        while let Some(key) = get_char_pressed() {
                            if !key.is_control() {
                                input.push(key);
                                // info!("{}",key);
                            }
                        }
                    } else {
                        // info!("{}", preedit);
                    }
                }
                if !input.is_empty() { self.insert(&input); }
            }
            
            if let Some(max) = self.max_length {
                let char_count = self.text.chars().count();
                if char_count > max {
                    let truncated: String = self.text.chars().take(max).collect();
                    self.text = truncated;
                    
                    let new_count = self.text.chars().count() as u32;
                    if self.cursor_pos > new_count {
                        self.cursor_pos = new_count;
                    }
                    self.selection = None;
                }
            }
        })();
        self
    }
    
    fn draw(&self, pos: impl Into<(f32,f32)>) {
        let (x, y) = pos.into();
        let bg = if self.hover || self.selected { self.fg } else { self.bg };
        let fg = if self.hover || self.selected { self.bg } else { self.fg };
        
        draw_rectangle((x, y), (self.width, self.height), bg);
        
        let size = self.height * 0.4;
        let padding = 4.0;
        let available_width = self.width - padding * 2.0;
        let display_text = self.display_text();
        
        let total_width = self.text_width(&display_text);
        
        let mut scroll_offset = 0.0;
        if total_width > available_width {
            let cursor_w = self.measure_prefix_width(display_text.chars().take(self.cursor_pos as usize));
            scroll_offset = (cursor_w - available_width * 0.5)
                .max(0.0)
                .min(total_width - available_width);
        }
        
        if self.selected {
            if let Some(sel) = self.selection {
                if !sel.is_empty() {
                    let s = sel.min() as usize;
                    let e = sel.max() as usize;
                    let start_x = self.measure_prefix_width(display_text.chars().take(s));
                    let sel_w = self.measure_prefix_width(display_text.chars().skip(s).take(e - s));
                    
                    let sx = x + padding + start_x - scroll_offset;
                    draw_rectangle((sx, y + 8.0), (sel_w, self.height - 16.0), Color::new(0.3, 0.5, 1.0, 0.5));
                }
            }
        }
        
        if self.text.is_empty() && self.placeholder.is_some() {
            let ph = self.placeholder.as_ref().unwrap();
            let dim = self.measure_ex(ph);
            let cx = x + self.width * 0.5 - dim.width * 0.5;
            let cy = y + self.height * 0.5 + dim.height * 0.25;
            
            draw_text_ex(
                ph,
                (cx, cy),
                TEXT_LB,
                TextParams {
                    font: self.font.clone(),
                    font_size: size,
                    font_scale: 1.0,
                    color: self.placeholder_color,
                    ..Default::default()
                }
            );
        } else {
            let text_color = if self.hover || self.selected { self.text_color } else { self.hovered_text_color };
            
            let mut visible_text = String::new();
            let mut accumulated_width = 0.0;
            for c in display_text.chars() {
                let char_w = self.measure_prefix_width(std::iter::once(c));
                if accumulated_width + char_w > available_width + scroll_offset {
                    break;
                }
                if accumulated_width >= scroll_offset {
                    visible_text.push(c);
                }
                accumulated_width += char_w;
            }
            
            let dim = self.measure_ex(&visible_text);
            let text_x = x + padding;
            let text_y = y + self.height * 0.5 + dim.height * 0.25;
            
            draw_text_ex(
                &visible_text,
                (text_x, text_y),
                TEXT_LB,
                TextParams {
                    font: self.font.clone(),
                    font_size: size,
                    font_scale: 1.0,
                    color: text_color,
                    ..Default::default()
                }
            );
            let preedit = &get_context().ime_preedit_string;
            if self.selected && !preedit.is_empty() {
                let base_x = text_x + self.measure_prefix_width(
                    display_text.chars().take(self.cursor_pos as usize)
                ) - scroll_offset;
                
                draw_text_ex(
                    preedit,
                    (base_x, text_y),
                    TEXT_LB,
                    TextParams {
                        font: self.font.clone(),
                        font_size: size,
                        color: text_color,
                        ..Default::default()
                    },
                );
                let w = self.text_width(preedit);
                draw_line(
                    (base_x, text_y + 3.0),
                    (base_x + w, text_y + 3.0),
                    1.0,
                    Color::new(0.7, 0.7, 0.7, 1.0),
                );
            }
        }
        
        if self.selected && self.cursor_visible {
            let cursor_w = self.measure_prefix_width(display_text.chars().take(self.cursor_pos as usize));
            let cx = x + padding + cursor_w - scroll_offset;
            draw_line((cx, y + 8.0), (cx, y + self.height - 8.0), 2.0, fg);
        }
        
        draw_rectangle_lines((x, y), (self.width, self.height), 2.0, fg);
    }
}

impl Action for TextInput {
    fn is_clicked(&self) -> bool {
        self.just_clicked
    }
    
    fn is_hovered(&self) -> bool {
        self.hover
    }
}