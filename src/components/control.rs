use types::*;
use specs::*;

#[derive(Component, Clone, Debug, Default)]
#[component(VecStorage)]
pub struct Control {
  pub forward: bool,
  pub backward: bool,
  pub turn_left: bool,
  pub turn_right: bool,
}