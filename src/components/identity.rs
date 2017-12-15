use specs::{Component, VecStorage};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Identity(usize);

impl Component for Identity {
  type Storage = VecStorage<Self>;
}

impl ::std::ops::Deref for Identity {
  type Target = usize;

  fn deref(&self) -> &usize {
    &self.0
  }
}

impl Into<usize> for Identity {
  fn into(self) -> usize {
    self.0
  }
}