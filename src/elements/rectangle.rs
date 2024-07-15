use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Buffer, Error, UIElement};

pub struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    horizontal_border: char,
    vertical_border: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    sub_elements: HashMap<String, Rc<RefCell<dyn UIElement>>>,
}

impl Rectangle {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
            top_left: ' ',
            top_right: ' ',
            bottom_left: ' ',
            bottom_right: ' ',
            vertical_border: ' ',
            horizontal_border: ' ',
            sub_elements: HashMap::new(),
        }
    }

    pub fn top_left(mut self, character: char) -> Self {
        self.top_left = character;
        self
    }
    pub fn top_right(mut self, character: char) -> Self {
        self.top_right = character;
        self
    }
    pub fn bottom_left(mut self, character: char) -> Self {
        self.bottom_left = character;
        self
    }
    pub fn bottom_right(mut self, character: char) -> Self {
        self.bottom_right = character;
        self
    }
    pub fn horizontal_border(mut self, character: char) -> Self {
        self.horizontal_border = character;
        self
    }
    pub fn vertical_border(mut self, character: char) -> Self {
        self.vertical_border = character;
        self
    }
}

impl UIElement for Rectangle {
    fn draw(&self, buffer: &mut Buffer) -> Result<(), Error> {
        buffer.draw_char(self.x, self.y, self.top_left)?;
        buffer.draw_char(self.x + self.width - 1, self.y, self.top_right)?;
        buffer.draw_char(self.x, self.y + self.height - 1, self.bottom_left)?;
        buffer.draw_char(
            self.x + self.width - 1,
            self.y + self.height - 1,
            self.bottom_right,
        )?;

        for i in 1..self.width - 1 {
            buffer.draw_char(self.x + i, self.y, self.horizontal_border)?;
            buffer.draw_char(self.x + i, self.y + self.height - 1, self.horizontal_border)?;
        }
        for i in 1..self.height - 1 {
            buffer.draw_char(self.x, self.y + i, self.vertical_border)?;
            buffer.draw_char(self.x + self.width - 1, self.y + i, self.vertical_border)?;
        }

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

    #[test]
    fn test_rectangle_new() {
        let rect = Rectangle::new(1, 1, 4, 3)
            .top_left('a')
            .top_right('b')
            .bottom_left('c')
            .bottom_right('d')
            .vertical_border('|')
            .horizontal_border('-');

        assert_eq!(rect.x, 1);
        assert_eq!(rect.y, 1);
        assert_eq!(rect.width, 4);
        assert_eq!(rect.height, 3);
        assert_eq!(rect.top_left, 'a');
        assert_eq!(rect.top_right, 'b');
        assert_eq!(rect.bottom_left, 'c');
        assert_eq!(rect.bottom_right, 'd');
        assert_eq!(rect.vertical_border, '|');
        assert_eq!(rect.horizontal_border, '-');
    }

    #[test]
    fn test_rectangle_draw() {
        let rect = Rectangle::new(1, 1, 4, 3)
            .top_left('a')
            .top_right('b')
            .bottom_left('c')
            .bottom_right('d')
            .vertical_border('|')
            .horizontal_border('-');

        let mut buffer = Buffer::new(10, 5);
        rect.draw(&mut buffer);
        let content = buffer.get_content();

        assert_eq!(content[1 + 1 * 10], 'a');
        assert_eq!(content[4 + 1 * 10], 'b');
        assert_eq!(content[1 + 3 * 10], 'c');
        assert_eq!(content[4 + 3 * 10], 'd');
    }

    #[test]
    fn test_set_position() {
        let mut rect = Rectangle::new(1, 1, 4, 3);
        rect.set_position(2, 2);
        assert_eq!(rect.x, 2);
        assert_eq!(rect.y, 2);
    }
}
