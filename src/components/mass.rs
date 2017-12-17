use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct Mass(pub f64);

impl ::std::ops::Deref for Mass {
  type Target = f64;
  fn deref(&self) -> &f64 {
    &self.0
  }
}

impl ::std::ops::DerefMut for Mass {
  fn deref_mut(&mut self) -> &mut f64 {
    &mut self.0
  }
}