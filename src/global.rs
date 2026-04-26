use std::collections::HashMap;
use std::any::{Any, TypeId};

static mut GLOBAL_POOL: Option<HashMap<String, Box<dyn Any>>> = None;

/// # Safety
/// 必须在程序启动时调用且仅调用一次。
/// 确保没有其他线程正在访问该变量。
pub unsafe fn init_global_pool() {
    if GLOBAL_POOL.is_some() {
        return;
    }
    GLOBAL_POOL = Some(HashMap::new());
}

/// 获取全局池的可变引用
///
/// # Safety
/// 调用者必须保证：
/// 1. `init_global_pool()` 已经被调用。
/// 2. 当前是单线程环境，或者没有其他并发访问。
pub unsafe fn get_global_pool_mut() -> &'static mut HashMap<String, Box<dyn Any>> {
    GLOBAL_POOL.as_mut().expect("Global pool not initialized! Call init_global_pool() first.")
}

/// 获取全局池的不可变引用
///
/// # Safety
/// 调用者必须保证：
/// 1. `init_global_pool()` 已经被调用。
/// 2. 当前是单线程环境，或者没有其他并发访问。
pub unsafe fn get_global_pool_ref() -> &'static HashMap<String, Box<dyn Any>> {
    GLOBAL_POOL.as_ref().expect("Global pool not initialized!")
}


/// 插入数据
/// 如果 Key 存在，旧值会被替换并 Drop
pub fn global_insert<T: 'static>(key: &str, value: T) {
    unsafe {
        let map = get_global_pool_mut();
        map.insert(key.to_string(), Box::new(value));
    }
}

/// 获取不可变引用
pub fn global_get<T: 'static>(key: &str) -> Option<&'static T> {
    unsafe {
        let map = get_global_pool_ref();
        map.get(key).and_then(|v| v.downcast_ref::<T>())
    }
}

/// 获取可变引用
pub fn global_get_mut<T: 'static>(key: &str) -> Option<&'static mut T> {
    unsafe {
        let map = get_global_pool_mut();
        map.get_mut(key).and_then(|v| v.downcast_mut::<T>())
    }
}

/// 移除数据
///
/// # Returns
/// 返回 `Option<Box<dyn Any>>`。
pub fn global_remove(key: &str) -> Option<Box<dyn Any>> {
    unsafe {
        let map = get_global_pool_mut();
        map.remove(key)
    }
}

/// 检查 Key 是否存在
pub fn global_contains(key: &str) -> bool {
    unsafe {
        let map = get_global_pool_ref();
        map.contains_key(key)
    }
}