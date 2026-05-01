//! 封装了超级简单的栈式场景
//!

use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem;
use anyhow::Context;
use crate::prelude::*;
use async_recursion::async_recursion;

static mut FLOW: Option<SceneFlow> = None;

pub async fn init_flow(main: impl Scene + BoxScene) -> anyhow::Result<()> {
    let flow = SceneFlow::new(main.boxed(), None).await?;
    unsafe {
        if FLOW.is_some() {
            panic!("Called `init_flow()` second time. To reset box, call `reset_flow()`")
        }
        FLOW = Some(flow);
    }
    Ok(())
}

pub async fn reset_flow(main: impl Scene + BoxScene) -> anyhow::Result<()> {
    let flow = SceneFlow::new(main.boxed(), None).await?;
    unsafe {
        if FLOW.is_none() {
            panic!("Called `reset_flow()` before `init_flow()`");
        }
        FLOW = Some(flow);
    }
    Ok(())
}

pub fn scene_flow() -> &'static mut SceneFlow {
    unsafe {
        if FLOW.is_none() {
            panic!("Called `scene_flow()` before `init_flow()`");
        }
        FLOW.as_mut().unwrap()
    }
}

#[macro_export]
macro_rules! scene_flow {
    (
        $scene:tt
    ) => {{
        use $crate::prelude::*;
        init_flow(($scene)).await.unwrap();
        
        loop {
            scene_flow().update().await.unwrap();
            scene_flow().draw().await.unwrap();
            
            if scene_flow().should_exit() { break; }
            next_frame().await;
        }
    }};
}


/// 当你确认你的场景的事件，传入中不应该input，用它吧
///
/// # Example
/// ```
/// pub struct MainScene;
///
/// impl_scene! { for MainScene {
///     async fn ready(&mut self, input: Option<Box<dyn Any>>) -> anyhow::Result<SceneAction> {
///         assert_no_input!(input);
///     }
/// }
/// ```
#[macro_export]
macro_rules! assert_no_input {
    ($input:expr) => {
        $input.map(|_| panic!("Asserted no input but got"));
    };
}

#[derive(Default)]
pub enum SceneAction {
    #[default]
    /// 什么也不做
    None,
    /// 打开子场景，本场景停止执行。
    Push(Box<dyn Scene>, Option<Box<dyn Any>>),
    /// 退出当前场景
    Pop(Option<Box<dyn Any>>),
    /// 退出多层场景
    PopMany(usize, Option<Box<dyn Any>>),
    /// 回到最底层场景
    Bottom(Option<Box<dyn Any>>),
    /// 退出进程
    Exit,
    /// 打开子场景，本场景继续执行（仅back和process）。
    Overlay(Box<dyn Scene>, Option<Box<dyn Any>>),
    /// 替换当前场景
    Replace(Box<dyn Scene>, Option<Box<dyn Any>>),
}

#[macro_export]
macro_rules! impl_scene {
    (for $ty:ty {$($tt:tt)*}) => {
        #[::async_trait::async_trait(?Send)]
        impl $crate::scene::Scene for $ty {
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
            $($tt)*
        }
        
        impl $crate::scene::BoxScene for $ty {
            fn boxed(self) -> Box<Self> { Box::new(self) }
        }
    };
}

pub trait BoxScene {
    fn boxed(self) -> Box<Self>;
}

#[async_trait(?Send)]
pub trait Scene: Any {
    fn as_any(&self) -> &dyn Any ;
    fn as_any_mut(&mut self) -> &mut dyn Any ;
    
    /// 你是准备吗
    async fn ready(&mut self, _input: Option<Box<dyn Any>>) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 底部绘制
    ///
    /// [`ui`](Scene::ui()) 前
    async fn back(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 每帧 ui 绘制
    ///
    /// [`process`](Scene::process()) 前
    ///
    /// 使用 [`handle.enroll()`](UiHandle::enroll()) 来登记组件id，这样就能在draw后绘制ui了！
    ///
    /// 当然你也可以使用 [`handle.give()`](UiHandle::give()) 来直接传递组件的所有权，同样也可以！
    async fn ui(&mut self, _handle: UiHandle) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 每帧逻辑
    async fn process(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 每帧绘制
    ///
    /// [`process`](Scene::process()) 后
    async fn draw(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 上层绘制
    ///
    /// 在 [`ui`](Scene::ui()) 绘制后执行
    async fn upper(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    async fn pause(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    async fn resume(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 子场景（直系）被pop后执行
    async fn on_pop(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 自己push后立马执行
    async fn on_push(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 自己overlay后立马执行
    async fn on_overlay(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 自己被pop前执行
    async fn on_death(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    /// 子场景（直系）被pop后的回传数据
    async fn on_result(&mut self, _child: Box<dyn Scene>, _result: Option<Box<dyn Any>>) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
    async fn next_scene(&mut self) -> anyhow::Result<SceneAction> { Ok(SceneAction::None) }
}

/// 场景流（实际上就是个栈）
pub struct SceneFlow {
    /// 要绘制的ui组件。
    widgets: Vec<(WidgetOption, (f32,f32))>,
    /// 登记的ui_box组件。
    enrollees: HashMap<u64, (f32, f32)>,
    stack: Vec<Box<dyn Scene>>,
    /// 存背景场景的索引（这些场景只执行back和process）
    back: Vec<usize>,
    paused: bool,
    should_exit: bool,
}

impl SceneFlow {
    pub async fn new(
        mut start_scene: Box<dyn Scene>,
        input: Option<Box<dyn Any>>
    ) -> anyhow::Result<Self> {
        let action = start_scene.ready(input).await?;
        
        let mut flow = Self {
            widgets: Vec::new(),
            enrollees: HashMap::new(),
            stack: vec![start_scene],
            back: vec![],
            paused: false,
            should_exit: false,
        };
        
        // 执行 ready 给出的动作
        flow.handle_single_action(action).await?;
        
        Ok(flow)
    }
    
    pub async fn update(&mut self) -> anyhow::Result<()> {
        if self.paused { return Ok(()); }
        
        self.handle_commands().await?;
        
        macro_rules! run {
            ($e: expr => $f: ident ($($args:expr)*) $end: block) => {
                let this = $e;
                if let Some(this) = this {
                    let act = this.$f ($($args)*).await?;
                    self.handle_single_action(
                        act
                    ).await?;
                } else $end
            };
        }
        
        
        for back in self.back.clone() {
            run!(self.stack.get_mut(back).map(|v|v.as_mut()) => back() {
                break;
            });
            run!(self.stack.get_mut(back).map(|v|v.as_mut()) => process() {
                break;
            });
        }
        
        run!(self.stack.last_mut().map(|v|v.as_mut()) => back() {
            self.should_exit = true;
            return Ok(());
        });
        run!(self.stack.last_mut().map(|v|v.as_mut()) => process() {
            self.should_exit = true;
            return Ok(());
        });
        
        let handle = UiHandle{
            enrollees: &mut self.enrollees,
            widgets: &mut self.widgets,
            _marker: PhantomData,
        };
        run!(self.stack.last_mut().map(|v|v.as_mut()) => ui(handle) {
            self.should_exit = true;
            return Ok(());
        });
        
        Ok(())
    }
    
    pub async fn draw(&mut self) -> anyhow::Result<()> {
        if self.paused { return Ok(()); }
        
        if let Some(top) = self.stack.last_mut() {
            let act = top.draw().await?;
            self.handle_single_action(act).await?;
        }
        
        let widgets = mem::take(&mut self.enrollees);
        widgets.into_iter().for_each(
            |(id, pos)| {
                ui_box()
                    .get_opt_mut(id)
                    .with_context(||format!("绘制时未找到Ui组件 {} ", id))
                    .unwrap()
                    .draw(pos);
            }
        );
        
        let widgets = mem::take(&mut self.widgets);
        widgets.into_iter().for_each(
            |(widget, pos)| {
                widget.draw(pos);
            }
        );
        
        if let Some(top) = self.stack.last_mut() {
            let act = top.upper().await?;
            self.handle_single_action(act).await?;
        }
        
        Ok(())
    }
    
    async fn handle_commands(&mut self) -> anyhow::Result<()> {
        if let Some(top) = self.stack.last_mut() {
            let cmd = top.next_scene().await?;
            self.handle_single_action(cmd).await?;
        }
        Ok(())
    }
    
    #[async_recursion(?Send)]
    async fn handle_single_action(&mut self, mut cmd: SceneAction) -> anyhow::Result<()> {
        loop {
            match cmd {
                SceneAction::None => break Ok(()),
                
                SceneAction::Exit => {
                    self.should_exit = true;
                    break Ok(());
                }
                
                SceneAction::Pop(res) => {
                    self.pop_chain(1, res).await?;
                    
                    self.cleanup_back();
                    break Ok(());
                }
                
                SceneAction::PopMany(n, res) => {
                    if n == 0 {
                        break Ok(());
                    }
                    self.pop_chain(n, res).await?;
                    
                    self.cleanup_back();
                    break Ok(());
                }
                
                SceneAction::Bottom(res) => {
                    let count = self.stack.len().saturating_sub(1);
                    self.pop_chain(count, res).await?;
                    
                    self.back.clear();
                    break Ok(());
                }
                
                SceneAction::Replace(mut scene, input) => {
                    self.stack.pop(); // 移除当前场景
                    
                    let act = scene.ready(input).await?;
                    
                    self.stack.push(scene);
                    
                    cmd = act;
                    continue;
                }
                
                SceneAction::Push(mut scene, input) => {
                    let act = scene.ready(input).await?;
                    
                    self.stack.push(scene);
                    
                    cmd = act;
                    continue;
                }
                
                // 6. 覆盖场景 (Overlay)
                // 记录背景 -> 准备新场景 -> 压入 -> **继续处理新场景的 Ready 返回值**
                SceneAction::Overlay(mut scene, input) => {
                    // 记录当前栈顶为背景
                    if !self.stack.is_empty() {
                        self.back.push(self.stack.len() - 1);
                    }
                    
                    // 准备新场景
                    let act = scene.ready(input).await?;
                    
                    self.stack.push(scene);
                    
                    // 关键：将 cmd 更新为新场景的返回值，继续循环
                    cmd = act;
                    continue;
                }
            }
        }
    }
    
    fn cleanup_back(&mut self) {
        let new_len = self.stack.len();
        let pos = self.back.partition_point(|&x| x < new_len);
        self.back.truncate(pos);
    }
    
    /// 链式弹出：每弹出一个，触发其 on_death + 父场景的 on_result
    /// - `count`: 要弹出的场景数量
    /// - `final_result`: 仅传递给**最后一个被弹出场景的父场景**，中间层传 None
    async fn pop_chain(
        &mut self,
        count: usize,
        mut final_result: Option<Box<dyn Any>>,
    ) -> anyhow::Result<()> {
        for i in 0..count {
            if self.stack.len() <= 1 {
                self.should_exit = true;
                break;
            }
            
            let mut child = self.stack.pop().unwrap();
            
            let death_act = child.on_death().await?;
            self.handle_single_action(death_act).await?;
            
            let result_to_parent = if i == count - 1 {
                final_result.take()
            } else {
                None
            };
            
            self.send_result(child, result_to_parent).await?;
            
            if let Some(parent) = self.stack.last_mut() {
                let resume_act = parent.resume().await?;
                self.handle_single_action(resume_act).await?;
            }
        }
        Ok(())
    }
    
    async fn send_result(
        &mut self,
        child: Box<dyn Scene>,
        result: Option<Box<dyn Any>>
    ) -> anyhow::Result<()> {
        if let Some(parent) = self.stack.last_mut() {
            let act = parent.on_result(child, result).await?;
            self.handle_single_action(act).await?;
        }
        Ok(())
    }
    
    pub async fn pause(&mut self) -> anyhow::Result<()> {
        self.paused = true;
        if let Some(top) = self.stack.last_mut() {
            let act = top.pause().await?;
            self.handle_single_action(act).await?;
        }
        Ok(())
    }
    
    pub async fn resume(&mut self) -> anyhow::Result<()> {
        self.paused = false;
        if let Some(top) = self.stack.last_mut() {
            let act = top.resume().await?;
            self.handle_single_action(act).await?;
        }
        Ok(())
    }
    
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}

pub struct UiHandle {
    pub(crate) enrollees: *mut HashMap<u64, (f32, f32)>,
    pub(crate) widgets: *mut Vec<(WidgetOption, (f32,f32))>,
    // 让编译器保证非多线程
    _marker: PhantomData<*mut ()>
}

impl UiHandle {
    pub(crate) fn enrollees_mut(&self) -> &mut  HashMap<u64, (f32, f32)> {
        unsafe {
            self.enrollees.as_mut().unwrap()
        }
    }
    
    pub(crate) fn widgets_mut(&self) -> &mut Vec<(WidgetOption, (f32,f32))> {
        unsafe {
            self.widgets.as_mut().unwrap()
        }
    }
    
    pub fn enroll(&self, id: u64, pos: impl ToPhysicalVec + 'static) -> Option<(f32, f32)> {
        self.enrollees_mut().insert(id, pos.to_physical_vec())
    }
    
    pub fn enroll_many(&self, map: HashMap<u64, (f32,f32)>) {
        self.enrollees_mut().extend(map)
    }
    
    pub fn give(&self, widget: impl IntoWidgetOption, pos: impl ToPhysicalVec + 'static) {
        self.widgets_mut().push((widget.upcast(),pos.to_physical_vec()))
    }
    
    pub fn give_many(&self, widgets: Vec<(WidgetOption, (f32,f32))>) {
        self.widgets_mut().extend(widgets)
    }
}