use types::*;
use specs::*;

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct LinearVelocity(pub Vector);

impl ::std::ops::Deref for LinearVelocity {
  type Target = Vector;
  fn deref(&self) -> &Vector {
    &self.0
  }
}

impl ::std::ops::DerefMut for LinearVelocity {
  fn deref_mut(&mut self) -> &mut Vector {
    &mut self.0
  }
}

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct AngularVelocity(pub Orientation);

impl ::std::ops::Deref for AngularVelocity {
  type Target = Orientation;
  fn deref(&self) -> &Orientation {
    &self.0
  }
}

impl ::std::ops::DerefMut for AngularVelocity {
  fn deref_mut(&mut self) -> &mut Orientation {
    &mut self.0
  }
}