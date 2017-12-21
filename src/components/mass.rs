use types::*;
use specs::*;

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct Mass(pub Precision);

impl ::std::ops::Deref for Mass {
  type Target = Precision;
  fn deref(&self) -> &Precision {
    &self.0
  }
}

impl ::std::ops::DerefMut for Mass {
  fn deref_mut(&mut self) -> &mut Precision {
    &mut self.0
  }
}