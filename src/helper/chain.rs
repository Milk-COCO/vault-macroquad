use crate::measure::ToPhysicalVec;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Chain<'s, T: 's> {
    start: T,
    ops: Vec<Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>>,
}

impl<'s, T: 's + Clone> Clone for Chain<'s, T> {
    fn clone(&self) -> Self {
        Self {
            start: self.start.clone(),
            ops: self.ops.clone(),
        }
    }
}

impl<'s,T: 's> Chain<'s, T> {
    /// 使用初始值创建链
    pub fn new( start: T ) -> Self {
        let this = Self { start, ops: vec![] };
        this
    }

    /// 此函数会克隆一个新的`Chain`，这意味着`T: Clone`，并：
    ///
    /// 添加一个操作。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn join(&self, op: impl Fn(T) -> T + 's) -> Self
    where T: Clone
    {
        let mut this = self.clone();
        this.ops.push(Rc::new(RefCell::new(Box::new(op))));
        this
    }

    /// 此函数会克隆一个新的`Chain`，这意味着`T: Clone`，并：
    ///
    /// 添加一个已存在的共享操作节点。
    ///
    /// 适用于多个 `Chain` 共享同一个变换逻辑的场景。
    pub fn join_raw(&self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self
    where T: Clone
    {
        let mut this = self.clone();
        this.ops.push(op);
        this
    }

    /// 为方便链式调用，此函数需要所有权，并：
    ///
    /// 添加一个操作。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn add(mut self, op: impl Fn(T) -> T + 's) -> Self {
        self.ops.push(Rc::new(RefCell::new(Box::new(op))));
        self
    }

    /// 为方便链式调用，此函数需要所有权，并：
    ///
    /// 添加一个已存在的共享操作节点。
    ///
    /// 适用于多个 `Chain` 共享同一个变换逻辑的场景。
    pub fn add_raw(mut self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> Self {
        self.ops.push(op);
        self
    }

    /// 添加一个操作。
    ///
    /// 该操作最终会被包裹在 `Rc<RefCell<Box<dyn Fn>>>` 中。
    pub fn push(&mut self, op: impl Fn(T) -> T + 's) -> &mut Self {
        self.ops.push(Rc::new(RefCell::new(Box::new(op))));
        self
    }

    /// 添加一个已存在的共享操作节点。
    ///
    /// 适用于多个 `Chain` 共享同一个变换逻辑的场景。
    pub fn push_raw(&mut self, op: Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>) -> &mut Self {
        self.ops.push(op);
        self
    }

    pub fn into_raw(self) -> (T,Vec<Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>>) {
        (self.start, self.ops)
    }

    pub fn from_raw(start: T, ops: Vec<Rc<RefCell<Box<dyn Fn(T) -> T + 's>>>>) -> Self {
        Self { start, ops }
    }

    /// 执行 `Chain`，不消耗初始值，这意味着`T: Clone`
    /// 
    /// # Panics
    /// 当内部引用的数据发生借用冲突时，panic！
    ///
    /// 当内部任意一个闭包panicked，panic！
    pub fn run(&self) -> T 
    where T: Clone{
        let mut v = self.start.clone();
        for expr in self.ops.iter() {
            v = expr.borrow()(v);
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
        let mut v = self.start;
        for expr in self.ops.into_iter() {
            v = expr.borrow()(v);
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