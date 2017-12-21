use types::*;
use specs::*;

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Player(pub usize);
