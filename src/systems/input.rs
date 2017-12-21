use components;
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
    ( WriteStorage<'a, components::Control> );

    fn run(&mut self, mut control: Self::SystemData) {
      for event in self.target.borrow_mut().events().iter() {
        for control in (&mut control).join() {
          if let WindowEvent::Key(code, _, Action::Press, _)  = event.value {
            match code {
              Key::W => {control.forward  = true;},
              Key::S => {control.backward = true;},
              Key::A => {control.turn_left     = true;},
              Key::D => {control.turn_right    = true;},
               _  => {}
            }
          } else if let WindowEvent::Key(code, _, Action::Release, _)  = event.value {
            match code {
              Key::W => {control.forward  = false;},
              Key::S => {control.backward = false;},
              Key::A => {control.turn_left     = false;},
              Key::D => {control.turn_right    = false;},
               _  => {}
            }
          }
        }
      }
    }
}