use crate::framework::traits::runtime::IRuntime;

pub struct GraphicsManager {}

impl IRuntime for GraphicsManager {
  fn initialize(&self) {}

  fn finalize(&self) {}

  fn tick(&self) {}
}
