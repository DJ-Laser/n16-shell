use std::fmt::Debug;

pub mod ipc;
pub mod multi_window;
pub mod single_window;

pub trait ShellMessage: Debug + Send + 'static {}

impl<T: Debug + Send + 'static> ShellMessage for T {}
