use super::runtime::IRuntime;
/// 用于抽象化不同平台的Application（并将其模块化）
pub trait IApplication: IRuntime {
  fn is_quit(&self) -> bool;
}
