use ::components::*;
use types::*;
use events;
use specs::{FetchMut, System, Join, ParJoin, ReadStorage, WriteStorage};
use nalgebra::core::{Matrix1,Vector1,Vector2};
use nalgebra::core::Unit;
use nalgebra;
use nalgebra::geometry::Translation2;
use std::time::{Duration, Instant};
use rayon::iter::ParallelIterator;
use  nalgebra::Rotation2;
use alga::linear::Similarity;
use num_complex::Complex;
use kiss3d::window::{Window, EventManager};
use glfw::{Action, WindowEvent, Key};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

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
            Key::W => {state.forward = true;},
            Key::A => {state.left    = true;},
            Key::D => {state.right   = true;},
             _  => {}
          }
        } else if let WindowEvent::Key(code, _, Action::Release, _)  = event.value {
          match code {
            Key::W => {state.forward = false;},
            Key::A => {state.left    = false;},
            Key::D => {state.right   = false;},
             _  => {}
          }
        }
      }
    }
}