use types::*;
use specs::*;

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct LinearForce(pub Vector);

impl ::std::ops::Deref for LinearForce {
  type Target = Vector;
  fn deref(&self) -> &Vector {
    &self.0
  }
}

impl ::std::ops::DerefMut for LinearForce {
  fn deref_mut(&mut self) -> &mut Vector {
    &mut self.0
  }
}

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct AngularForce(pub Orientation);

impl ::std::ops::Deref for AngularForce {
  type Target = Orientation;
  fn deref(&self) -> &Orientation {
    &self.0
  }
}

impl ::std::ops::DerefMut for AngularForce {
  fn deref_mut(&mut self) -> &mut Orientation {
    &mut self.0
  }
}