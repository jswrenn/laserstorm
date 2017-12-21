use types::*;
use specs::*;

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct LinearAcceleration(pub Vector);

impl ::std::ops::Deref for LinearAcceleration {
  type Target = Vector;
  fn deref(&self) -> &Vector {
    &self.0
  }
}

impl ::std::ops::DerefMut for LinearAcceleration {
  fn deref_mut(&mut self) -> &mut Vector {
    &mut self.0
  }
}

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct AngularAcceleration(pub Orientation);

impl ::std::ops::Deref for AngularAcceleration {
  type Target = Orientation;
  fn deref(&self) -> &Orientation {
    &self.0
  }
}

impl ::std::ops::DerefMut for AngularAcceleration {
  fn deref_mut(&mut self) -> &mut Orientation {
    &mut self.0
  }
}