use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;
use std::marker::PhantomData;
use std::ops::Deref;
use std::any::Any;
use crate::prelude::{Button, IntoWidgetOption, Label, Widget, WidgetOption, Widown};

pub mod widgets;

// ============ 全局 UiBox 管理 ============

static mut UI_BOX: Option<UiBox> = None;

pub fn init_box() {
    let ui = UiBox::new();
    unsafe {
        if UI_BOX.is_some() {
            panic!("Called `init_box()` second time. To refresh box, call `reset_box()`")
        }
        UI_BOX = Some(ui);
    }
}

pub fn reset_box() {
    let ui = UiBox::new();
    unsafe {
        if UI_BOX.is_none() {
            panic!("Called `reset_box()` before `init_box()`");
        }
        UI_BOX = Some(ui);
    }
}

pub fn ui_box() -> &'static mut UiBox {
    unsafe {
        if UI_BOX.is_none() {
            panic!("Called `ui_box()` before `init_box()`");
        }
        UI_BOX.as_mut().unwrap()
    }
}

// ============ 辅助类型 ============

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct GetWidgetOption<T>(Option<T>);

impl<T> GetWidgetOption<T> {
    pub fn unwrap(self) -> T {
        self.0.expect("Called `GetWidgetOption::unwrap()` on a `None` value")
    }
}

// ============ UiBox 主结构 ============

pub struct UiBox {
    pub widgets: HashMap<HashOption, Option<WidgetOption>>,
}

impl UiBox {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
        }
    }
    
    /// 插入组件
    ///
    /// ✅ 支持传任意 `IntoHashOption` 类型：
    /// - `"id"` (&str), `"id".to_string()` (String), `42u32`, `&42u32` 等
    pub fn insert(
        &mut self,
        id: impl IntoHashOption,
        widget: impl IntoWidgetOption
    ) -> Option<WidgetOption> {
        self.widgets.insert(id.upcast(), Some(widget.upcast())).flatten()
    }
    
    /// 获取组件的不可变引用
    ///
    /// ✅ 支持传任意 `IntoHashOption` 类型
    pub fn widget_ref<T: Widown>(&self, id: impl IntoHashOption) -> Option<&T> {
        let key = id.upcast_clone();
        self.widgets.get(&key)?.as_ref()?.downcast_ref::<T>()
    }
    
    /// 获取组件的可变引用
    pub fn widget_mut<T: Widown>(&mut self, id: impl IntoHashOption) -> Option<&mut T> {
        let key = id.upcast_clone();
        self.widgets.get_mut(&key)?.as_mut()?.downcast_mut::<T>()
    }
    
    /// 临时获取组件所有权进行操作
    ///
    /// - `None`: 找不到对应组件
    /// - `Some(None)`: 闭包返回 `Ok(new_widget)`，新组件放回原位
    /// - `Some(Some(err))`: 闭包返回 `Err(err)`，组件被删除，返回错误
    pub fn widget<T: Widown, R>(
        &mut self,
        id: impl IntoHashOption,
        op: impl FnOnce(T) -> Result<T, R>
    ) -> Option<Option<R>> {
        let key = id.upcast_clone();
        let w = self.widgets.get_mut(&key)?;
        
        Some({
            // 类型检查
            w.as_ref()?.downcast_ref::<T>()?;
            
            // 取出所有权
            let widget = mem::replace(w, None)?
                .downcast::<T>()
                .ok()?;
            
            match op(widget) {
                Ok(new_widget) => {
                    let _ = mem::replace(w, Some(new_widget.upcast()));
                    None
                }
                Err(re) => {
                    self.widgets.remove(&key);
                    Some(re)
                }
            }
        })
    }
    
    /// 删除组件
    pub fn remove(&mut self, id: impl IntoHashOption) -> Option<WidgetOption> {
        let key = id.upcast_clone();
        self.widgets.remove(&key).flatten()
    }
    
    /// 检查组件是否存在
    pub fn contains(&self, id: impl IntoHashOption) -> bool {
        let key = id.upcast_clone();
        self.widgets.contains_key(&key)
    }
}

// ============ 标记 Trait ============

/// 标记 trait：标识可用作 HashOption 变体的具体类型
pub trait Hadown: Hash {}

// ============ HashId: 类型安全的 ID 包装器（可选，零克隆） ============

/// 类型安全的哈希 ID 包装器
///
/// - 内部持有 `HashOption`，通过 `Deref` 提供 `&HashOption` 访问
/// - 泛型 `T` 用于编译时类型追踪，运行时无开销
/// - 用于零克隆的 `HashMap` 查找：`&HashId<T>` → `&HashOption`（通过 Deref）
///
/// # 使用场景
/// 高频查询的 ID（如每帧更新的血条），可预先创建 `HashId` 缓存起来，避免重复克隆
pub struct HashId<T>(HashOption, PhantomData<T>);

impl<T> HashId<T> {
    /// 从具体类型创建 `HashId`
    ///
    /// # Example
    /// ```ignore
    /// let id = HashId::new("my_btn".to_string());  // HashId<String>
    /// ```
    pub fn new(val: T) -> Self
    where
        T: Into<HashOption>
    {
        Self(val.into(), PhantomData)
    }
    
    /// 消耗 `self`，返回内部的 `HashOption`
    pub fn into_inner(self) -> HashOption {
        self.0
    }
}

// 🔑 关键：实现 Deref，让 &HashId<T> 自动转为 &HashOption（零克隆查找）
impl<T> Deref for HashId<T> {
    type Target = HashOption;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 便捷转换：T → HashId<T>
impl<T> From<T> for HashId<T>
where
    T: Into<HashOption>
{
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

// HashId 可克隆（克隆内部的 HashOption）
impl<T> Clone for HashId<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

// Hash/PartialEq/Eq: 委托给内部的 HashOption
impl<T> Hash for HashId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> PartialEq for HashId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for HashId<T> {}

// 调试输出
impl<T> std::fmt::Debug for HashId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HashId").field(&self.0).finish()
    }
}

// ============ IntoHashOption Trait（核心：啥都能传） ============

/// 标记 trait：可转换为 HashOption 的类型
///
/// ✅ 支持传：
/// - 所有权: `"id".to_string()`, `42u32`
/// - 引用: `&"id".to_string()`, `&42u32`
/// - 字面量: `"id"` (如果实现了 `&str` 支持)
///
/// # 设计原则
/// - 小类型（u32/bool）：`clone()` 是位复制，零开销
/// - 大类型（String）：一次克隆，高频场景可缓存 `HashId`
pub trait IntoHashOption: Clone + Into<HashOption> {
    /// 转换为 `HashOption`（消耗所有权）
    fn upcast(self) -> HashOption;
    
    /// 🔑 克隆后转换：用于查找时临时创建 key
    ///
    /// - Copy 类型（u32/bool）：零开销位复制
    /// - 非 Copy 类型（String）：一次克隆，可接受
    fn upcast_clone(&self) -> HashOption {
        self.clone().upcast()
    }
    
    /// 可选：创建类型安全的 `HashId`（零克隆查找）
    ///
    /// # Example
    /// ```ignore
    /// let id = "btn".to_string().as_hash_id();  // HashId<String>
    /// // 后续使用 &id 查找，零克隆
    /// ```
    fn as_hash_id(self) -> HashId<Self>
    where
        Self: Sized,
    {
        HashId::new(self)
    }
}

// ============ HashOption 枚举（宏生成） ============

macro_rules! impl_hash_option {
    ($($ty:ty => $variant:ident),*$(,)*) => {
        
        /// 通用哈希选项枚举
        ///
        /// 支持 `Any` 下转 + `Hash` + `Eq`，用于 HashMap key
        #[derive(Eq, PartialEq, Clone, Debug)]
        pub enum HashOption {
            $($variant($ty)),*
        }

        impl HashOption {
            /// 转为 `&dyn Any`（用于类型反射）
            pub fn as_any(&self) -> &dyn Any {
                match self {
                    $(Self::$variant(v) => v),*
                }
            }
            
            /// 转为 `&mut dyn Any`
            pub fn as_any_mut(&mut self) -> &mut dyn Any {
                match self {
                    $(Self::$variant(v) => v),*
                }
            }
            
            /// 下转到具体类型引用
            pub fn downcast_ref<T: 'static + Hadown>(&self) -> Option<&T> {
                self.as_any().downcast_ref::<T>()
            }
            
            /// 下转到具体类型可变引用
            pub fn downcast_mut<T: 'static + Hadown>(&mut self) -> Option<&mut T> {
                self.as_any_mut().downcast_mut::<T>()
            }
            
            /// 检查是否为某个具体类型
            pub fn is<T: 'static>(&self) -> bool {
                self.as_any().type_id() == std::any::TypeId::of::<T>()
            }
        }
        
        impl Hash for HashOption {
            fn hash<H: Hasher>(&self, state: &mut H) {
                match self {
                    $(Self::$variant(v) => v.hash(state)),*
                }
            }
        }
        
        
        $(
            impl Hadown for $ty {}
            
            // ✅ 所有权转换
            impl IntoHashOption for $ty {
                fn upcast(self) -> HashOption { self.into() }
            }
            impl From<$ty> for HashOption {
                fn from(val: $ty) -> Self { HashOption::$variant(val) }
            }
            
            // ✅ 🔑 修复：给 &ty 也实现 From 和 IntoHashOption
            impl From<&$ty> for HashOption {
                fn from(val: &$ty) -> Self {
                    HashOption::$variant(val.clone())
                }
            }
            
            impl IntoHashOption for &$ty {
                fn upcast(self) -> HashOption {
                    self.clone().into()
                }
            }
        )*
    };
}

// 注册支持的 ID 类型
impl_hash_option!(
    String => String,
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    usize => Usize,
    isize => Isize,
    bool => Bool,
);


impl From<&str> for HashOption {
    fn from(val: &str) -> Self {
        HashOption::String(val.to_string())
    }
}

impl IntoHashOption for &str {
    fn upcast(self) -> HashOption {
        self.into()
    }
}