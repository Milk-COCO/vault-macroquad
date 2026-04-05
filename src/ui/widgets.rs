//! 组件构成ui
//!
//! # 思想
//! 这些都是组件们的构造器。实际上是内部调用传入的 `&mut `[`Ui`] 节点 的相关方法来绘制我们能看到的组件。
//!
//! 具体地，定义完成后，调用各自的 `ui` 函数来传入 `&mut `[`Ui`] 节点，
//! 然后内部进行一些杂七杂八的操作，构造出真实的组件。
//! 然后这个函数可能会返回一些状态，具体是什么看各自节点的实际用途
//!
//! 组件大致可以分别为两种组件，容器组件和普通组件
//!
//! ## 容器
//! 说白了就是可以用来装组件的组件。
//!
//! 我们有三种容器，分别是 [`Group`]、[`Window`]、[`TreeNode`]
//!
//! 他们的作用实际上是基于 “域”。定义完成，消耗构造器、开启域，然后就知道接下来的东西该放在这个域里面啦。最后得通过各自的 Token 类型标记域的结束。
//!
//! 那我们怎么知道谁是和上一帧相同的那一个容器？很简单，让构造器明确自己要构造的是谁，所以他们的 `new` 函数都需要一个 [`Id`]。使用 [`crate::hash!`] 宏来创建唯一 Id
//!
//! ##### 比如
//! 比方说，我们调用一个实例的 [`Window::begin`] 构造窗口并开启窗口域，返回 [`WindowToken`]，然后我们就进行要放在这个窗口里面的组件的构造，最后调用 [`WindowToken::end`] 标记域已结束
//!
//! 所以得按顺序来结束域，不然会发生什么我也不知道。
//!
//! 当然，这仨部分操作有封装。比如 [`Window::ui`] 传入 节点和要执行的操作（在这里面构造窗口内部的东西），最后返回当前窗口的是否要存在的状态。其它容器大同小异
//!
//!
//! ## 普通组件
//! 没什么特别的，详见各自的定义吧
//!
mod button;
mod checkbox;
mod combobox;
mod drag;
mod editbox;
mod group;
mod input;
mod label;
mod popup;
mod progress_bar;
mod separator;
mod slider;
mod tabbar;
mod texture;
mod tree_node;
mod window;

pub use button::Button;
pub use checkbox::Checkbox;
pub use combobox::ComboBox;
pub use editbox::Editbox;
pub use group::{Group, GroupToken};
pub use input::InputText;
pub use label::Label;
pub use popup::Popup;
pub use progress_bar::ProgressBar;
pub use slider::Slider;
pub use tabbar::Tabbar;
pub use texture::Texture;
pub use tree_node::{TreeNode, TreeNodeToken};
pub use window::{Window, WindowToken};
