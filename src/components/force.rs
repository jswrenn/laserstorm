use types::{Vector, Orientation};
use specs::{Component, VecStorage};

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct LinearForce(pub Vector<f64>);

impl ::std::ops::Deref for LinearForce {
  type Target = Vector<f64>;
  fn deref(&self) -> &Vector<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for LinearForce {
  fn deref_mut(&mut self) -> &mut Vector<f64> {
    &mut self.0
  }
}

#[derive(Component, Clone, Debug)]
#[component(VecStorage)]
pub struct AngularForce(pub Orientation<f64>);

impl ::std::ops::Deref for AngularForce {
  type Target = Orientation<f64>;
  fn deref(&self) -> &Orientation<f64> {
    &self.0
  }
}

impl ::std::ops::DerefMut for AngularForce {
  fn deref_mut(&mut self) -> &mut Orientation<f64> {
    &mut self.0
  }
}