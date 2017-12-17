use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct Identity(pub usize);

impl ::std::ops::Deref for Identity {
  type Target = usize;
  fn deref(&self) -> &usize {
    &self.0
  }
}

impl ::std::ops::DerefMut for Identity {
  fn deref_mut(&mut self) -> &mut usize {
    &mut self.0
  }
}