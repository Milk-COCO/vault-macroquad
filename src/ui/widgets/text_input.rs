//! This module defines the [`TextInput`] widget that allows the user to enter text.
use std::any::Any;
use std::sync::Arc;
use std::time::Instant;
use parking_lot::RwLock;
use crate::prelude::*;
use super::{Action, Widget};

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
                // 第一次按下
                self.pressed = true;
                self.first_press_time = Some(now);
                self.last_repeat_time = None;
                true
            } else {
                // 持续按下，检查重复
                let first = self.first_press_time.unwrap();
                let initial_delay = 0.5; // 初始延迟 500ms
                let repeat_interval = 0.05; // 重复间隔 50ms
                
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
    
    bg: Color,
    fg: Color,
    placeholder: Option<String>,
    placeholder_color: Color,
    
    hover: bool,
    just_clicked: bool,
    selected: bool,
    
    font: Option<Arc<RwLock<Font>>>,
    width: f32,
    height: f32,
    
    cursor_blink_timer: f32,
    cursor_visible: bool,
    
    is_password: bool,
    max_length: Option<usize>,
    
    // 屏幕位置（用于输入法）
    screen_pos: (f32, f32),
    screen_cursor_pos: (f32, f32),
    
    // 拖拽选择
    is_dragging: bool,
    
    // 回车事件
    just_submitted: bool,
    
    // 双击检测
    last_click_time: Option<Instant>,
    last_click_pos: (f32, f32),
    
    // 键盘重复输入状态
    key_left: KeyRepeatState,
    key_right: KeyRepeatState,
    key_backspace: KeyRepeatState,
    key_delete: KeyRepeatState,
}

impl TextInput {
    /// Creates a new [`TextInput`] widget.
    pub fn new(width: f32, height: f32, bg: Color, fg: Color, font: Option<Arc<RwLock<Font>>>) -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
            selection: None,
            
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
            max_length: None,
            
            screen_pos: (0.0, 0.0),
            screen_cursor_pos: (0.0, 0.0),
            
            is_dragging: false,
            just_submitted: false,
            
            last_click_time: None,
            last_click_pos: (0.0, 0.0),
            
            key_left: KeyRepeatState::new(),
            key_right: KeyRepeatState::new(),
            key_backspace: KeyRepeatState::new(),
            key_delete: KeyRepeatState::new(),
        }
    }
    
    // --- Builder 模式 ---
    
    pub fn with_placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    
    pub fn with_password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }
    
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }
    
    // --- 公开 API ---
    
    /// Returns the text entered in the [`TextInput`] widget.
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    
    /// Sets the text.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.cursor_pos = self.text.len() as u32;
        self.selection = None;
    }
    
    /// Clears the text.
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
        self.selection = None;
    }
    
    /// Returns true if the user just pressed Enter.
    pub fn is_submitted(&self) -> bool {
        self.just_submitted
    }
    
    /// Returns the cursor position on screen (for IME).
    pub fn ime_cursor_screen_pos(&self) -> (f32, f32) {
        self.screen_cursor_pos
    }
    
    // --- 内部辅助函数 ---
    
    fn display_text(&self) -> String {
        if self.is_password {
            "*".repeat(self.text.len())
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
    
    fn char_width(&self, c: char, size: f32) -> f32 {
        measure_text(&c.to_string(), self.font.clone(), size, 1.0).width
    }
    
    fn text_width(&self, text: &str, size: f32) -> f32 {
        measure_text(text, self.font.clone(), size, 1.0).width
    }
    
    // 手动实现双击检测
    fn check_double_click(&mut self, mouse_pos: (f32, f32)) -> bool {
        let now = Instant::now();
        let (mx, my) = mouse_pos;
        
        if let Some(last_time) = self.last_click_time {
            let (last_x, last_y) = self.last_click_pos;
            let time_diff = now.duration_since(last_time).as_millis();
            let pos_diff = ((mx - last_x).powi(2) + (my - last_y).powi(2)).sqrt();
            
            // 300ms 内 + 5像素 内算双击
            if time_diff < 300 && pos_diff < 5.0 {
                self.last_click_time = None;
                return true;
            }
        }
        
        self.last_click_time = Some(now);
        self.last_click_pos = mouse_pos;
        false
    }
    
    // 删除选中内容
    fn delete_selection(&mut self) {
        if let Some(sel) = self.selection {
            if !sel.is_empty() {
                let start = sel.min() as usize;
                let end = sel.max() as usize;
                self.text.replace_range(start..end, "");
                self.cursor_pos = start as u32;
                self.selection = None;
            }
        }
    }
    
    // 插入字符
    fn insert_char(&mut self, c: char) {
        // 检查长度限制
        if let Some(max) = self.max_length {
            if self.text.len() >= max {
                return;
            }
        }
        
        self.text.insert(self.cursor_pos as usize, c);
        self.cursor_pos += 1;
        self.cursor_visible = true;
        self.cursor_blink_timer = 0.0;
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
    
    fn process(&mut self, pos: impl Into<(f32,f32)>) {
        let (x, y) = pos.into();
        self.screen_pos = (x, y);
        
        let mouse_pos = mouse_position();
        let mx = mouse_pos.0;
        let my = mouse_pos.1;
        
        let size = self.height * 0.4;
        let padding = 4.0;
        let _available_width = self.width - padding * 2.0;
        
        // 重置一次性事件
        self.just_clicked = false;
        self.just_submitted = false;
        
        // 悬浮检测
        self.hover = mx >= x && mx <= x + self.width && my >= y && my <= y + self.height;
        
        let clicked = is_mouse_button_pressed(MouseButton::Left);
        let mouse_down = is_mouse_button_down(MouseButton::Left);
        
        // 点击/拖拽逻辑
        if clicked && self.hover {
            self.just_clicked = true;
            self.selected = true;
            self.is_dragging = true;
            
            // 检查双击
            let is_double_click = self.check_double_click(mouse_pos);
            
            // 计算点击位置对应的字符位置
            let display_text = self.display_text();
            let mut click_x = mx - x - padding;
            
            // 计算滚动偏移
            let text_width = self.text_width(&display_text, size);
            let available_width = self.width - padding * 2.0;
            let mut scroll_offset = 0.0;
            if text_width > available_width {
                let cursor_x = self.text_width(&display_text[..self.cursor_pos as usize], size);
                scroll_offset = (cursor_x - available_width / 2.0).max(0.0).min(text_width - available_width);
            }
            
            click_x += scroll_offset;
            
            // 找到最接近的字符位置
            let mut current_x = 0.0;
            let mut new_pos = 0;
            for (i, c) in display_text.chars().enumerate() {
                let char_w = self.char_width(c, size);
                if click_x < current_x + char_w / 2.0 {
                    break;
                }
                current_x += char_w;
                new_pos = i + 1;
            }
            
            self.cursor_pos = new_pos as u32;
            
            // 双击选中单词
            if is_double_click {
                let start = self.find_word_boundary(self.cursor_pos, false);
                let end = self.find_word_boundary(self.cursor_pos, true);
                self.selection = Some(Selection { start, end });
            } else {
                self.selection = Some(Selection::new(self.cursor_pos));
            }
            
            self.cursor_visible = true;
            self.cursor_blink_timer = 0.0;
        } else if clicked && !self.hover {
            self.selected = false;
            self.is_dragging = false;
            self.selection = None;
        }
        
        // 拖拽选择
        if self.is_dragging && mouse_down {
            let display_text = self.display_text();
            let mut drag_x = mx - x - padding;
            
            let text_width = self.text_width(&display_text, size);
            let available_width = self.width - padding * 2.0;
            let mut scroll_offset = 0.0;
            if text_width > available_width {
                let cursor_x = self.text_width(&display_text[..self.cursor_pos as usize], size);
                scroll_offset = (cursor_x - available_width / 2.0).max(0.0).min(text_width - available_width);
            }
            
            drag_x += scroll_offset;
            
            let mut current_x = 0.0;
            let mut new_pos = 0;
            for (i, c) in display_text.chars().enumerate() {
                let char_w = self.char_width(c, size);
                if drag_x < current_x + char_w / 2.0 {
                    break;
                }
                current_x += char_w;
                new_pos = i + 1;
            }
            
            if let Some(ref mut sel) = self.selection {
                sel.end = new_pos as u32;
            }
            self.cursor_pos = new_pos as u32;
        } else if !mouse_down {
            self.is_dragging = false;
        }
        
        // ESC 取消选中
        if self.selected && is_key_pressed(KeyCode::Escape) {
            self.selected = false;
            self.selection = None;
        }
        
        // 键盘输入处理
        if self.selected {
            // 光标闪烁
            self.cursor_blink_timer += get_frame_time();
            if self.cursor_blink_timer > 0.5 {
                self.cursor_blink_timer = 0.0;
                self.cursor_visible = !self.cursor_visible;
            }
            
            // 修饰键
            let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
            let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
            
            // --- 左移 ---
            if self.key_left.update(is_key_down(KeyCode::Left)) {
                if ctrl {
                    self.cursor_pos = self.find_word_boundary(self.cursor_pos, false);
                } else {
                    self.cursor_pos = self.cursor_pos.saturating_sub(1);
                }
                
                if !shift {
                    self.selection = None;
                } else if let Some(ref mut sel) = self.selection {
                    sel.end = self.cursor_pos;
                } else {
                    self.selection = Some(Selection { start: self.cursor_pos + 1, end: self.cursor_pos });
                }
                
                self.cursor_visible = true;
                self.cursor_blink_timer = 0.0;
            }
            
            // --- 右移 ---
            if self.key_right.update(is_key_down(KeyCode::Right)) {
                if ctrl {
                    self.cursor_pos = self.find_word_boundary(self.cursor_pos, true);
                } else {
                    self.cursor_pos = (self.cursor_pos + 1).min(self.text.len() as u32);
                }
                
                if !shift {
                    self.selection = None;
                } else if let Some(ref mut sel) = self.selection {
                    sel.end = self.cursor_pos;
                } else {
                    self.selection = Some(Selection { start: self.cursor_pos - 1, end: self.cursor_pos });
                }
                
                self.cursor_visible = true;
                self.cursor_blink_timer = 0.0;
            }
            
            // --- Home ---
            if is_key_pressed(KeyCode::Home) {
                self.cursor_pos = 0;
                if !shift {
                    self.selection = None;
                } else if let Some(ref mut sel) = self.selection {
                    sel.end = 0;
                }
                self.cursor_visible = true;
                self.cursor_blink_timer = 0.0;
            }
            
            // --- End ---
            if is_key_pressed(KeyCode::End) {
                self.cursor_pos = self.text.len() as u32;
                if !shift {
                    self.selection = None;
                } else if let Some(ref mut sel) = self.selection {
                    sel.end = self.text.len() as u32;
                }
                self.cursor_visible = true;
                self.cursor_blink_timer = 0.0;
            }
            
            // --- 退格 ---
            if self.key_backspace.update(is_key_down(KeyCode::Backspace)) {
                self.delete_selection();
                
                if self.cursor_pos > 0 {
                    self.text.remove(self.cursor_pos as usize - 1);
                    self.cursor_pos -= 1;
                }
                
                self.cursor_visible = true;
                self.cursor_blink_timer = 0.0;
            }
            
            // --- 删除 ---
            if self.key_delete.update(is_key_down(KeyCode::Delete)) {
                self.delete_selection();
                
                if self.cursor_pos < self.text.len() as u32 {
                    self.text.remove(self.cursor_pos as usize);
                }
                
                self.cursor_visible = true;
                self.cursor_blink_timer = 0.0;
            }
            
            // --- 回车 ---
            if is_key_pressed(KeyCode::Enter) {
                self.just_submitted = true;
            }
            
            // --- 普通字符输入 ---
            while let Some(key) = get_char_pressed() {
                // 过滤控制字符（只保留可打印字符）
                if !key.is_control() {
                    self.delete_selection();
                    self.insert_char(key);
                }
            }
        }
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
        let text_width = self.text_width(&display_text, size);
        
        // 计算滚动偏移
        let mut scroll_offset = 0.0;
        if text_width > available_width {
            let cursor_x = self.text_width(&display_text[..self.cursor_pos as usize], size);
            scroll_offset = (cursor_x - available_width / 2.0).max(0.0).min(text_width - available_width);
        }
        
        // 绘制选中背景
        if self.selected {
            if let Some(sel) = self.selection {
                if !sel.is_empty() {
                    let start = sel.min() as usize;
                    let end = sel.max() as usize;
                    
                    let sel_start_x = self.text_width(&display_text[..start], size);
                    let sel_width = self.text_width(&display_text[start..end], size);
                    
                    let sel_x = x + padding + sel_start_x - scroll_offset;
                    let sel_y = y + 8.0;
                    let sel_h = self.height - 16.0;
                    
                    draw_rectangle((sel_x, sel_y), (sel_width, sel_h), Color::new(0.3, 0.5, 1.0, 0.5));
                }
            }
        }
        
        // 绘制文字或 Placeholder
        if self.text.is_empty() && self.placeholder.is_some() {
            let placeholder = self.placeholder.as_ref().unwrap();
            let placeholder_size = measure_text(placeholder, self.font.clone(), size, 1.0);
            
            draw_text_ex(
                placeholder,
                (x + self.width / 2.0 - placeholder_size.width / 2.0, y + self.height / 2.0 + placeholder_size.height / 4.0),
                (-1.,-1.),
                TextParams {
                    font: self.font.clone(),
                    font_size: size,
                    font_scale: 1.0,
                    color: self.placeholder_color,
                    ..Default::default()
                }
            );
        } else {
            // 裁剪文字
            let mut visible_text = String::new();
            let mut visible_width = 0.0;
            
            for c in display_text.chars() {
                let char_w = self.char_width(c, size);
                if visible_width + char_w > available_width + scroll_offset {
                    break;
                }
                if visible_width >= scroll_offset {
                    visible_text.push(c);
                }
                visible_width += char_w;
            }
            
            let visible_text_size = measure_text(&visible_text, self.font.clone(), size, 1.0);
            
            // 计算文字起始位置
            let text_x = x + padding - scroll_offset;
            let text_y = y + self.height / 2.0 + visible_text_size.height / 4.0;
            
            draw_text_ex(
                &visible_text,
                (text_x, text_y),
                (-1.,-1.),
                TextParams {
                    font: self.font.clone(),
                    font_size: size,
                    font_scale: 1.0,
                    color: fg,
                    ..Default::default()
                }
            );
        }
        
        // 绘制光标
        if self.selected && self.cursor_visible {
            let cursor_text = &display_text[..self.cursor_pos as usize];
            let cursor_text_width = self.text_width(cursor_text, size);
            
            let cursor_x = x + padding + cursor_text_width - scroll_offset;
            let cursor_y = y + 8.0;
            let cursor_h = self.height - 16.0;
            
            draw_line((cursor_x, cursor_y), (cursor_x, cursor_y + cursor_h), 3.0, fg);
        }
        
        draw_rectangle_lines((x, y), (self.width, self.height), 4.0, fg);
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