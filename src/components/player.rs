use specs::{Component, VecStorage};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Player(usize);

impl Component for Player {
  type Storage = VecStorage<Self>;
}
