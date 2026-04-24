use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::Deref;
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

/// 一个通用的 UI 组件标识符。
/// 由 `TypeId` (区分类型) 和 `u64` (该类型的哈希值) 组成。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiId {
    type_id: TypeId,
    hash: u64,
}

impl UiId {
    /// 从任何实现了 Any + Hash 的类型创建 UiId
    pub fn new<T: Any + Hash + ?Sized>(value: &T) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut hasher);
        Self {
            type_id: TypeId::of::<T>(),
            hash: hasher.finish(),
        }
    }
}

/// 用于缓存 ID 的结构体，避免每帧重复计算哈希。
#[derive(Debug, Clone, Copy)]
pub struct CachedId<T: 'static>(UiId, PhantomData<T>);

impl<T: Any + Hash> CachedId<T> {
    pub fn new(value: &T) -> Self {
        Self(UiId::new(value), PhantomData)
    }
    
    pub fn as_id(&self) -> UiId {
        self.0
    }
}

impl<T: Any + Hash> Deref for CachedId<T> {
    type Target = UiId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ToUiId {
    fn to_ui_id(&self) -> UiId;
}

macro_rules! impl_to_ui_id_for_integers {
    ($($ty:ty),*) => {
        $(
            impl ToUiId for $ty {
                fn to_ui_id(&self) -> UiId { UiId::new(self) }
            }
            impl ToUiId for &$ty {
                fn to_ui_id(&self) -> UiId { UiId::new(*self) }
            }
        )*
    };
}

impl_to_ui_id_for_integers!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, bool, str, String);

impl ToUiId for UiId {
    fn to_ui_id(&self) -> UiId { *self }
}

impl<T: Any + Hash> ToUiId for CachedId<T> {
    fn to_ui_id(&self) -> UiId { self.0 }
}

pub struct UiBox {
    widgets: HashMap<UiId, Option<WidgetOption>>,
}

macro_rules! impl_convenient_get {
    ($name:ident, $ty:ty) => {
        /// 获取指定类型的组件，如果不存在则 Panic
        pub fn $name(&mut self, id: impl ToUiId) -> &mut $ty {
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
        id: impl ToUiId,
        widget: impl IntoWidgetOption
    ) -> Option<WidgetOption> {
        self.widgets.insert(id.to_ui_id(), Some(widget.upcast())).flatten()
    }
    
    /// 获取组件的可变引用
    pub fn get_mut<W: 'static + Widown>(&mut self, id: impl ToUiId) -> Option<&mut W> {
        let key = id.to_ui_id();
        self.widgets.get_mut(&key)?.as_mut()?.downcast_mut::<W>()
    }
    
    /// 获取组件的不可变引用
    pub fn get<W: 'static + Widown>(&self, id: impl ToUiId) -> Option<&W> {
        let key = id.to_ui_id();
        self.widgets.get(&key)?.as_ref()?.downcast_ref::<W>()
    }
    
    /// 删除组件并返回其所有权
    pub fn remove(&mut self, id: impl ToUiId) -> Option<WidgetOption> {
        let key = id.to_ui_id();
        self.widgets.remove(&key)?
    }
    
    /// 检查组件是否存在
    pub fn contains(&self, id: impl ToUiId) -> bool {
        let key = id.to_ui_id();
        self.widgets.contains_key(&key)
    }
    
    
    impl_convenient_get!(button, Button);
    impl_convenient_get!(label, Label);
    impl_convenient_get!(input, TextInput);
    impl_convenient_get!(container, Container);
    impl_convenient_get!(picture, Picture);
}