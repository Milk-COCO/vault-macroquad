use crate::measure::ToPhysicalVec;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};

/// 链式计算的起始点。
///
/// 可以是具体的值，也可以是一个生成值的工厂函数。
#[derive(Clone)]
pub enum ChainHead<'s, T: 's> {
    /// 静态初始值
    Value(T),
    /// 动态初始值生成器
    Fn(Rc<RefCell<Box<dyn Fn() -> T + 's>>>),
}

/// 链式计算的一个部分
///
/// 可为Rc闭包或Weak闭包
///
/// Weak闭包（执行时）会被自动置为None
#[derive(Clone)]
pub enum ChainUnit<'s, T: 's> {
    Owned(Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>),
    Shared(Weak<RefCell<Box<dyn Fn(T) -> T + 's>>>),
    None
}

impl<'s, T> ChainUnit<'s, T> {
    /// 得到Rc
    pub fn upgrade(&self) -> Option<Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>> {
        match self {
            ChainUnit::Owned(rc) => Some(rc.clone()),
            ChainUnit::Shared(wk) => wk.upgrade(),
            ChainUnit::None => None
        }
    }
}

impl<'s, T> Debug for ChainUnit<'s, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "ChainUnit::{:?}",
            match self {
                ChainUnit::Owned(_) => "Owned",
                ChainUnit::Shared(_) => "Shared",
                ChainUnit::None => "None"
            }
        ).as_str())
    }
}

impl<'s, T> Debug for ChainHead<'s, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "ChainHead::{:?}",
            match self {
                ChainHead::Value(_) => "Value",
                ChainHead::Fn(_) => "Fn"
            }
        ).as_str())
    }
}

impl<'s, T: 's> ChainHead<'s, T> {
    /// 若为值，clone值；
    ///
    /// 若为闭包，执行闭包；
    fn run(&self) -> T
    where T: Clone {
        match self {
            ChainHead::Value(v) => v.clone(),
            ChainHead::Fn(f) => f.borrow()(),
        }
    }

    fn cost(self) -> T {
        match self {
            ChainHead::Value(v) => v,
            ChainHead::Fn(f) => f.borrow()(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chain<'s, T: 's> {
    start: ChainHead<'s,T>,
    front: Vec<ChainUnit<'s, T>>,
    back: Vec<ChainUnit<'s, T>>,
}

impl<'s,T: 's> Chain<'s, T> {
    pub fn new_with(op: impl Fn() -> T + 's) -> Self {
        Self {
            start: ChainHead::Fn(Rc::new(RefCell::new(Box::new(op)))),
            front: vec![],
            back: vec![]
        }
    }

    pub fn new_with_raw(op: Rc<RefCell<Box<dyn Fn() -> T + 's>>>) -> Self {
        Self {
            start: ChainHead::Fn(op),
            front: vec![],
            back: vec![]
        }
    }

    /// 使用初始值创建链
    pub fn new( start: T ) -> Self {
        let this = Self { start: ChainHead::Value(start), front: vec![], back: vec![] };
        this
    }

    // --- Back Operations (Renamed) ---

    /// 此函数会克隆一个新的`Chain`，这意味着`T: Clone`，并：
    ///
    /// 添加一个操作到后链。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn join_back(&self, op: impl Fn(T) -> T + 's) -> Self
    where T: Clone
    {
        let mut this = self.clone();
        this.back.push(ChainUnit::Owned(Rc::new(RefCell::new(Box::new(op)))));
        this
    }

    /// 此函数会克隆一个新的`Chain`，这意味着`T: Clone`，并：
    ///
    /// 添加一个已存在的弱引用共享操作节点到后链。
    pub fn join_shared_back(&self, op: Weak<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self
    where T: Clone{
        let mut this = self.clone();
        this.back.push(ChainUnit::Shared(op));
        this
    }

    /// 此函数会克隆一个新的`Chain`，这意味着`T: Clone`，并：
    ///
    /// 添加一个已存在的共享操作节点到后链。
    pub fn join_owned_back(&self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self
    where T: Clone
    {
        let mut this = self.clone();
        this.back.push(ChainUnit::Owned(op));
        this
    }

    /// 为方便链式调用，此函数需要所有权，并：
    ///
    /// 添加一个操作到后链。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn add_back(mut self, op: impl Fn(T) -> T + 's) -> Self {
        self.back.push(ChainUnit::Owned(Rc::new(RefCell::new(Box::new(op)))));
        self
    }

    /// 为方便链式调用，此函数需要所有权，并：
    ///
    /// 添加一个已存在的共享操作节点到后链。
    pub fn add_shared_back(mut self, op: Weak<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self {
        self.back.push(ChainUnit::Shared(op));
        self
    }

    /// 为方便链式调用，此函数需要所有权，并：
    ///
    /// 添加一个已存在的共享操作节点到后链。
    pub fn add_owned_back(mut self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self {
        self.back.push(ChainUnit::Owned(op));
        self
    }

    /// 添加一个操作到后链。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn push_back(&mut self, op: impl Fn(T) -> T + 's) -> &mut Self {
        self.back.push(ChainUnit::Owned(Rc::new(RefCell::new(Box::new(op)))));
        self
    }

    /// 添加一个已存在的弱引用共享操作节点到后链。
    pub fn push_shared_back(&mut self, op: Weak<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> &mut Self {
        self.back.push(ChainUnit::Shared(op));
        self
    }

    /// 添加一个已存在的共享操作节点到后链。
    pub fn push_owned_back(&mut self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> &mut Self {
        self.back.push(ChainUnit::Owned(op));
        self
    }

    // --- Front Operations (New) ---

    /// 此函数会克隆一个新的`Chain`，这意味着`T: Clone`，并：
    ///
    /// 添加一个操作到前链。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn join_front(&self, op: impl Fn(T) -> T + 's) -> Self
    where T: Clone
    {
        let mut this = self.clone();
        this.front.push(ChainUnit::Owned(Rc::new(RefCell::new(Box::new(op)))));
        this
    }

    /// 此函数会克隆一个新的`Chain`，这意味着`T: Clone`，并：
    ///
    /// 添加一个已存在的弱引用共享操作节点到前链。
    pub fn join_shared_front(&self, op: Weak<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self
    where T: Clone{
        let mut this = self.clone();
        this.front.push(ChainUnit::Shared(op));
        this
    }

    /// 此函数会克隆一个新的`Chain`，这意味着`T: Clone`，并：
    ///
    /// 添加一个已存在的共享操作节点到前链。
    pub fn join_owned_front(&self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self
    where T: Clone
    {
        let mut this = self.clone();
        this.front.push(ChainUnit::Owned(op));
        this
    }

    /// 为方便链式调用，此函数需要所有权，并：
    ///
    /// 添加一个操作到前链。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn add_front(mut self, op: impl Fn(T) -> T + 's) -> Self {
        self.front.push(ChainUnit::Owned(Rc::new(RefCell::new(Box::new(op)))));
        self
    }

    /// 为方便链式调用，此函数需要所有权，并：
    ///
    /// 添加一个已存在的共享操作节点到前链。
    pub fn add_shared_front(mut self, op: Weak<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self {
        self.front.push(ChainUnit::Shared(op));
        self
    }

    /// 为方便链式调用，此函数需要所有权，并：
    ///
    /// 添加一个已存在的共享操作节点到前链。
    pub fn add_owned_front(mut self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self {
        self.front.push(ChainUnit::Owned(op));
        self
    }

    /// 添加一个操作到前链。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn push_front(&mut self, op: impl Fn(T) -> T + 's) -> &mut Self {
        self.front.push(ChainUnit::Owned(Rc::new(RefCell::new(Box::new(op)))));
        self
    }

    /// 添加一个已存在的弱引用共享操作节点到前链。
    pub fn push_shared_front(&mut self, op: Weak<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> &mut Self {
        self.front.push(ChainUnit::Shared(op));
        self
    }

    /// 添加一个已存在的共享操作节点到前链。
    pub fn push_owned_front(&mut self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> &mut Self {
        self.front.push(ChainUnit::Owned(op));
        self
    }

    // --- Raw Access ---

    pub fn into_raw(self) -> (ChainHead<'s,T>, Vec<ChainUnit<'s, T>>, Vec<ChainUnit<'s, T>>) {
        (self.start, self.front, self.back)
    }

    pub fn from_raw(start: ChainHead<'s,T>, front: Vec<ChainUnit<'s, T>>, back: Vec<ChainUnit<'s, T>>) -> Self {
        Self { start, front, back }
    }

    pub fn raw_front(&self) -> &Vec<ChainUnit<'s, T>> {
        &self.front
    }

    pub fn raw_front_mut(&mut self) -> &mut Vec<ChainUnit<'s, T>> {
        &mut self.front
    }

    pub fn raw_back(&self) -> &Vec<ChainUnit<'s, T>> {
        &self.back
    }

    pub fn raw_back_mut(&mut self) -> &mut Vec<ChainUnit<'s, T>> {
        &mut self.back
    }

    /// 执行 `Chain`，不消耗初始值，这意味着`T: Clone`
    ///
    /// # Panics
    /// 当内部引用的数据发生借用冲突时，panic！
    ///
    /// 当内部任意一个闭包panicked，panic！
    pub fn run(&self) -> T
    where T: Clone{
        let mut v = self.start.run();

        // Execute front operations in order
        for expr in self.front.iter() {
            match expr.upgrade(){
                Some(expr) => v = expr.borrow()(v),
                None => {},
            }
        }

        // Execute back operations in order
        for expr in self.back.iter() {
            match expr.upgrade(){
                Some(expr) => v = expr.borrow()(v),
                None => {},
            }
        }
        v
    }

    /// 消耗 `Chain`
    ///
    /// # Panics
    /// 当内部引用的数据发生借用冲突时，panic！
    ///
    /// 当内部任意一个闭包panicked，panic！
    pub fn cost(self) -> T {
        let mut v = self.start.cost();

        // Execute front operations in order
        for expr in self.front.into_iter() {
            match expr.upgrade(){
                Some(expr) => v = expr.borrow()(v),
                None => {},
            }
        }

        // Execute back operations in order
        for expr in self.back.into_iter() {
            match expr.upgrade(){
                Some(expr) => v = expr.borrow()(v),
                None => {},
            }
        }
        v
    }
}

impl<'s, T: ToPhysicalVec + 's + Clone> ToPhysicalVec for Chain<'s, T> {
    /// # Panics
    /// 当内部引用的数据发生借用冲突时，panic！
    ///
    /// 当内部任意一个闭包panicked，panic！
    fn to_physical_vec(&self) -> (f32, f32) {
        self.run().to_physical_vec()
    }
}