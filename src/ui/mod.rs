use std::collections::HashMap;
use crate::prelude::*;

pub mod widgets;

static mut UI_BOX: Option<UiBox> = None;

pub fn init_box() {
    unsafe {
        if UI_BOX.is_some() {
            panic!("Called `init_box()` second time. To reset box, call `reset_box()`");
        }
        UI_BOX = Some(UiBox::new());
    }
}

pub fn reset_box() {
    unsafe {
        if UI_BOX.is_none() {
            panic!("Called `reset_box()` before `init_box()`");
        }
        UI_BOX = Some(UiBox::new());
    }
}

/// 获取全局 UiBox 引用
pub fn ui_box() -> &'static mut UiBox {
    unsafe {
        UI_BOX.as_mut().expect("Called `ui_box()` before `init_box()`")
    }
}

pub struct UiBox {
    widgets: HashMap<u64, Option<WidgetOption>>,
}

macro_rules! impl_convenient_get {
    ($name:ident, $ty:ty) => {
        /// 获取指定类型的组件，如果不存在则 Panic
        pub fn $name(&mut self, id: u64) -> &mut $ty {
            self.get_mut::<$ty>(id).unwrap_or_else(|| {
                panic!("Widget of type {} not found", stringify!($ty))
            })
        }
    };
}

impl UiBox {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
        }
    }
    
    /// 插入组件
    pub fn insert(
        &mut self,
        id: u64,
        widget: impl IntoWidgetOption
    ) -> Option<WidgetOption> {
        self.widgets.insert(id, Some(widget.upcast())).flatten()
    }
    
    /// 获取组件的可
    pub(crate) fn get_opt_mut(&mut self, id: u64) -> Option<&mut WidgetOption> {
        self.widgets.get_mut(&id)?.as_mut()
    }
    
    /// 获取组件的不可变引用
    pub(crate) fn get_opt(&self, id: u64) -> Option<&WidgetOption> {
        self.widgets.get(&id)?.as_ref()
    }
    
    
    /// 获取组件的可变引用
    pub fn get_mut<W: 'static + Widown>(&mut self, id: u64) -> Option<&mut W> {
        self.widgets.get_mut(&id)?.as_mut()?.downcast_mut::<W>()
    }
    
    /// 获取组件的不可变引用
    pub fn get<W: 'static + Widown>(&self, id: u64) -> Option<&W> {
        self.widgets.get(&id)?.as_ref()?.downcast_ref::<W>()
    }
    
    /// 删除组件并返回其所有权
    pub fn remove(&mut self, id: u64) -> Option<WidgetOption> {
        self.widgets.remove(&id)?
    }
    
    /// 检查组件是否存在
    pub fn contains(&self, id: u64) -> bool {
        self.widgets.contains_key(&id)
    }
    
    
    impl_convenient_get!(button, Button);
    impl_convenient_get!(label, Label);
    impl_convenient_get!(input, TextInput);
    impl_convenient_get!(container, Container);
    impl_convenient_get!(picture, Picture);
}

/// 只是调用了 [`const_fnv1a_hash::fnv1a_hash_str_64()`]
/// 封装成宏是因为想要使用literal元变量的特性，强调它是编译时可知。
#[macro_export]
macro_rules! id {
    ($s:literal) => {
        $crate::const_fnv1a_hash::fnv1a_hash_str_64($s)
    };
}

pub use id;