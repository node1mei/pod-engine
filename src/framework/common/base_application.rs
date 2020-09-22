use crate::framework::traits::{application::IApplication, runtime::IRuntime};

pub struct BaseApplication {
  pub b_quit: bool,
}

impl BaseApplication {
  pub fn new() -> BaseApplication {
    return BaseApplication { b_quit: false };
  }
}

impl IApplication for BaseApplication {
  fn is_quit(&self) -> bool {
    return self.b_quit;
  }
}

impl IRuntime for BaseApplication {
  fn initialize(&self) {}

  fn finalize(&self) {}

  fn tick(&self) {}
}
