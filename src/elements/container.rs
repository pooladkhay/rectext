use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Buffer, Error, UIElement};

pub struct Container {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    sub_elements: HashMap<String, Rc<RefCell<dyn UIElement>>>,
}

impl Container {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
            sub_elements: HashMap::new(),
        }
    }
}

impl UIElement for Container {
    fn draw(&self, buffer: &mut Buffer) -> Result<(), Error> {
        for elem in self.sub_elements.values() {
            elem.borrow().draw(buffer)?;
        }
        Ok(())
    }
    fn set_position(&mut self, x: usize, y: usize) {
        for sub in self.sub_elements.values_mut() {
            let (sub_x, sub_y) = sub.borrow().get_position();
            let diff_x = self.x.abs_diff(sub_x);
            let diff_y = self.y.abs_diff(sub_y);

            sub.borrow_mut().set_position(x + diff_x, y + diff_y)
        }

        self.x = x;
        self.y = y;
    }
    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    fn add_sub_element(&mut self, id: String, element: Rc<RefCell<dyn UIElement>>) {
        let (x, y) = element.borrow().get_position();
        element.borrow_mut().set_position(x + self.x, y + self.y);
        self.sub_elements.insert(id, element);
    }
    fn remove_sub_element(&mut self, id: &str) {
        self.sub_elements.remove(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::Text;

    #[test]
    fn test_new_container() {
        let container = Container::new(0, 0, 100, 100);
        assert_eq!(container.get_position(), (0, 0));
        assert_eq!(container.sub_elements.len(), 0);
    }

    #[test]
    fn test_add_sub_element() {
        let mut container = Container::new(0, 0, 100, 100);
        let sub_element = Rc::new(RefCell::new(Text::new(10, 10, "Text")));
        container.add_sub_element("sub1".to_string(), sub_element.clone());
        assert_eq!(container.sub_elements.len(), 1);
        assert_eq!(sub_element.borrow().get_position(), (10, 10));
    }

    #[test]
    fn test_set_position() {
        let mut container = Container::new(0, 0, 100, 100);
        let sub_element = Rc::new(RefCell::new(Text::new(10, 10, "Text")));
        container.add_sub_element("sub1".to_string(), sub_element.clone());
        container.set_position(20, 20);
        assert_eq!(container.get_position(), (20, 20));
        assert_eq!(sub_element.borrow().get_position(), (30, 30));
    }

    #[test]
    #[should_panic]
    fn test_set_position_panic() {
        struct FaultyElement;
        impl UIElement for FaultyElement {
            fn draw(&self, _buffer: &mut Buffer) -> Result<(), Error> {
                Ok(())
            }
            fn set_position(&mut self, _x: usize, _y: usize) {
                panic!("Panic on set_position");
            }
            fn get_position(&self) -> (usize, usize) {
                (0, 0)
            }
            fn add_sub_element(&mut self, _id: String, _element: Rc<RefCell<dyn UIElement>>) {}
            fn remove_sub_element(&mut self, _id: &str) {}
        }

        let mut container = Container::new(0, 0, 100, 100);
        let faulty_element = Rc::new(RefCell::new(FaultyElement));
        container.add_sub_element("faulty".to_string(), faulty_element);
        container.set_position(10, 10); // This should panic
    }
}
