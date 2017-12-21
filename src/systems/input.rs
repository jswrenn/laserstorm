use components::*;
use events;
use specs::*;
use kiss3d::window::Window;
use glfw::{Action, WindowEvent, Key};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Input<'w> {
  target: Rc<RefCell<&'w mut Window>>,
}

impl<'w> Input<'w> {
  pub fn new(window : Rc<RefCell<&'w mut Window>>,) -> Input {
    Input {
      target: window
    }
  }
}

impl<'a,'w> System<'a> for Input<'w> {
  type SystemData =
    ( FetchMut<'a, events::ControlState> );

    fn run(&mut self, mut state: Self::SystemData) {
      for event in self.target.borrow_mut().events().iter() {
        if let WindowEvent::Key(code, _, Action::Press, _)  = event.value {
          match code {
            Key::W => {state.forward  = true;},
            Key::S => {state.backward = true;},
            Key::A => {state.left     = true;},
            Key::D => {state.right    = true;},
             _  => {}
          }
        } else if let WindowEvent::Key(code, _, Action::Release, _)  = event.value {
          match code {
            Key::W => {state.forward  = false;},
            Key::S => {state.backward = false;},
            Key::A => {state.left     = false;},
            Key::D => {state.right    = false;},
             _  => {}
          }
        }
      }
    }
}