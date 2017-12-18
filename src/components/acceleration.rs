use types::{Vector, Orientation};
use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct LinearAcceleration(pub Vector<f64>);

impl ::std::ops::Deref for LinearAcceleration {
  type Target = Vector<f64>;
  fn deref(&self) -> &Vector<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for LinearAcceleration {
  fn deref_mut(&mut self) -> &mut Vector<f64> {
    &mut self.0
  }
}

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct AngularAcceleration(pub Orientation<f64>);

impl ::std::ops::Deref for AngularAcceleration {
  type Target = Orientation<f64>;
  fn deref(&self) -> &Orientation<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for AngularAcceleration {
  fn deref_mut(&mut self) -> &mut Orientation<f64> {
    &mut self.0
  }
}