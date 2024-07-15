use std::{cell::RefCell, rc::Rc};

use crate::{Buffer, Error};

pub trait UIElement {
    fn draw(&self, buffer: &mut Buffer) -> Result<(), Error>;
    fn get_position(&self) -> (usize, usize);
    fn set_position(&mut self, x: usize, y: usize);
    fn add_sub_element(&mut self, id: String, element: Rc<RefCell<dyn UIElement>>);
    fn remove_sub_element(&mut self, id: &str);
}
