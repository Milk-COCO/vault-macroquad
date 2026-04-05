use crate::{
    math::{vec2, Rect, Vec2},
    ui::{ElementState, Id, Ui, UiContent, WindowContext},
};

/// [`WindowContext`] 的构造器。
///
/// 构造一个显示在程序内部的窗口，当然不是系统原生窗口。
///
/// [`Window::new`] 一个窗口，预设要进行的操作，并调用 [`Window::begin`] 来构造 [`WindowToken`]。或者使用 [`Window::ui`] 来快捷插入组件。
#[derive(Debug, Clone)]
pub struct Window {
    id: Id,
    position: Vec2,
    size: Vec2,
    close_button: bool,
    movable: bool,
    titlebar: bool,
    label: Option<String>,
}

impl Window {
    /// 创建窗口，其中位置仅在初次调用有效，大小随时生效。
    pub fn new<V: Into<Vec2>>(id: Id, position: V, size: V) -> Window {
        let position = position.into();
        let size = size.into();
        Window {
            id,
            position,
            size,
            close_button: false,
            movable: true,
            titlebar: true,
            label: None,
        }
    }

    /// 窗口标题（未设置时不显示标题）
    ///
    /// 如果标题栏显示，会（默认居中）显示在标题栏上。
    ///
    /// see [`Window::titlebar`]
    pub fn label(self, label: &str) -> Window {
        Window {
            label: Some(label.to_string()),
            ..self
        }
    }

    /// 可被用户拖动（默认 `true`）
    ///
    /// 此属性只在第一次设置时有效。
    pub fn movable(self, movable: bool) -> Window {
        Window { movable, ..self }
    }
    
    
    /// 是否显示关闭按钮（默认 `false`）
    ///
    /// 关闭按钮不占空间。结果在 [`Window::ui`] 的返回值提供
    pub fn close_button(self, close_button: bool) -> Window {
        Window {
            close_button,
            ..self
        }
    }

    /// 是否显示标题栏（默认 `true`）
    ///
    /// 标题栏占空间。并且是否真正地占空间取决于第一次设置。
    // TODO: plzz fix the fooing bar
    pub fn titlebar(self, titlebar: bool) -> Window {
        Window { titlebar, ..self }
    }

    /// 快速构造 Window 上下文，并将内容封入 [`Ui`] 节点
    ///
    /// 提供 [`Ui`]，依次调用 [`Window::begin`]、 传入的`f`、[`WindowToken::end`]
    ///
    /// 并返回当前是否需要保持开启（返回 `false` 即为当前是 关闭按钮 被按下的第一时刻）。
    ///
    pub fn ui<F: FnOnce(&mut Ui)>(self, ui: &mut Ui, f: F) -> bool {
        let token = self.begin(ui);
        f(ui);
        token.end(ui)
    }
    
    /// 给提供的 [`Ui`] 节点构造 Window 上下文，将其加入
    ///
    /// 内部调用 [`Ui::begin_window`] ，传入当前 [`Window`] 的数据来开启窗口域。
    ///
    /// 最后你需要调用 [`WindowToken::end`] 来结束域。
    ///
    /// [`Window::ui`] 是 begin-end 的简易封装。
    pub fn begin(self, ui: &mut Ui) -> WindowToken {
        let context = ui.begin_window(
            self.id,
            None,
            self.position,
            self.size,
            self.titlebar,
            self.movable,
        );

        // TODO: this will make each new window focused(appeared on the top) always
        // consider adding some configuration to be able to spawn background windows
        if context.window.was_active == false {
            ui.focus_window(self.id);
        }

        let mut context = ui.get_active_window_context();

        self.draw_window_frame(&mut context);
        if self.close_button && self.draw_close_button(&mut context) {
            context.close();
        }

        let clip_rect = context.window.content_rect();
        context.scroll_area();

        context.window.painter.clip(clip_rect);

        WindowToken
    }

    fn draw_close_button(&self, context: &mut WindowContext) -> bool {
        let style = context.style;
        let size = Vec2::new(style.title_height - 4., style.title_height - 4.);
        let pos = Vec2::new(
            context.window.position.x + context.window.size.x - style.title_height + 1.,
            context.window.position.y + 2.,
        );
        let rect = Rect::new(pos.x, pos.y, size.x, size.y);
        let (hovered, clicked) = context.register_click_intention(rect);

        context.window.painter.draw_element_background(
            &context.style.button_style,
            pos,
            size,
            ElementState {
                focused: context.focused,
                hovered,
                clicked: hovered && context.input.is_mouse_down,
                selected: false,
            },
        );

        clicked
    }

    fn draw_window_frame(&self, context: &mut WindowContext) {
        let focused = context.focused;
        let style = context.style;
        let position = context.window.position;
        let size = context.window.size;

        context.window.painter.draw_element_background(
            &style.window_style,
            position,
            size,
            ElementState {
                focused,
                hovered: false,
                clicked: false,
                selected: false,
            },
        );

        // TODO: figure what does title bar mean with windows with background
        if self.titlebar {
            if let Some(label) = &self.label {
                context.window.painter.draw_element_content(
                    &context.style.window_titlebar_style,
                    position,
                    vec2(size.x, style.title_height),
                    &UiContent::Label(label.into()),
                    ElementState {
                        focused,
                        clicked: false,
                        hovered: false,
                        selected: false,
                    },
                );
            }
            context.window.painter.draw_line(
                vec2(position.x, position.y + style.title_height),
                vec2(position.x + size.x, position.y + style.title_height),
                style.window_titlebar_style.color(ElementState {
                    focused,
                    clicked: false,
                    hovered: false,
                    selected: false,
                }),
            );
        }
    }
}

#[must_use = "Must call `.end()` to finish Window"]
pub struct WindowToken;

impl WindowToken {
    /// 结束 Window域，返回当前是否需要保持开启（返回 `false` 即为当前是 关闭按钮 被按下的第一时刻）。
    pub fn end(self, ui: &mut Ui) -> bool {
        let context = ui.get_active_window_context();
        context.window.painter.clip(None);

        let opened = context.window.want_close == false;

        ui.end_window();

        opened
    }
}

impl Ui {
    pub fn window<V: Into<Vec2>, F: FnOnce(&mut Ui)>(&mut self, id: Id, position: V, size: V, f: F) -> bool {
        Window::new(id, position, size).titlebar(false).ui(self, f)
    }
}
