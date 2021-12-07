use super::cell::Grid;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ControlFlow {
  Continue,
  Stop,
  Wait(u128),
}

#[derive(Debug, Clone, Copy)]
pub struct EventLoop<S> {
  state: S,
  control: ControlFlow,
}

impl EventLoop<Grid> {
  pub fn new(state: Grid) -> Self {
    Self {
      state,
      control: ControlFlow::Continue,
    }
  }

  pub fn run<T: Fn(&mut Grid, &mut ControlFlow) -> ()>(mut self, callback: T) {
    loop {
      let time = Instant::now();
      match self.control {
        ControlFlow::Continue => callback(&mut self.state, &mut self.control),
        ControlFlow::Stop => std::process::abort(),
        ControlFlow::Wait(millis) => {
          while time.elapsed().as_millis() < millis {}
          callback(&mut self.state, &mut self.control);
        }
      }
    }
  }
}
