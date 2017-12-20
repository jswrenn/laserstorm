use specs::{Component, VecStorage};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ControlState {
  pub forward   : bool,
  pub backward  : bool,
  pub left      : bool,
  pub right     : bool,
}


