use std::error::Error;
pub trait IRuntime {
  /// 初始化模块
  fn initialize(&self);

  /// 用来在模块结束的时候打扫战场的
  fn finalize(&self);

  /// 用来让驱动模块驱动该模块执行的。每调用一次，模块进行一个单位的处理
  fn tick(&self);
}
