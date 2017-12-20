use specs::{Component, VecStorage};

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Player(pub usize);
