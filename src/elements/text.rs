use crate::{Buffer, Error, UIElement};

pub struct Text {
    x: usize,
    y: usize,
    content: String,
}

impl Text {
    pub fn new(x: usize, y: usize, content: &str) -> Self {
        Self {
            x,
            y,
            content: content.to_string(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
}

impl UIElement for Text {
    fn draw(&self, buffer: &mut Buffer) -> Result<(), Error> {
        for (i, c) in self.content.chars().enumerate() {
            buffer.draw_char(self.x + i, self.y, c)?;
        }
        Ok(())
    }
    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn add_sub_element(
        &mut self,
        _id: String,
        _element: std::rc::Rc<std::cell::RefCell<dyn UIElement>>,
    ) {
        todo!()
    }

    fn remove_sub_element(&mut self, _id: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_new() {
        let text = Text::new(2, 2, "Hello");
        assert_eq!(text.x, 2);
        assert_eq!(text.y, 2);
        assert_eq!(text.content, "Hello".to_string());
    }

    #[test]
    fn test_text_draw() -> Result<(), Error> {
        let text = Text::new(2, 2, "Hello");
        let mut buffer = Buffer::new(10, 5);
        text.draw(&mut buffer)?;
        let content = buffer.get_content();

        assert_eq!(content[2 + 2 * 10], 'H');
        assert_eq!(content[3 + 2 * 10], 'e');
        assert_eq!(content[4 + 2 * 10], 'l');
        assert_eq!(content[5 + 2 * 10], 'l');
        assert_eq!(content[6 + 2 * 10], 'o');
        Ok(())
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_draw_text_off_screen() {
        let text = Text::new(2, 2, "Hello");
        let mut buffer = Buffer::new(5, 3);
        text.draw(&mut buffer);

        let content = buffer.get_content();
        // panic!("{:?}", content);
        assert_eq!(content[2 + 2 * 5], 'H');
        assert_eq!(content[3 + 2 * 5], 'e');
        assert_eq!(content[4 + 2 * 5], 'l');
        assert_eq!(content[5 + 2 * 5], 'l');
        // assert_eq!(content[5 + 2 * 5], ' ');
    }

    #[test]
    fn test_set_content() {
        let mut text = Text::new(2, 2, "Hello");
        text.set_content("World");
        assert_eq!(text.content, "World".to_string());
    }

    #[test]
    fn test_set_position() {
        let mut text = Text::new(2, 2, "Hello");
        text.set_position(3, 3);
        assert_eq!(text.x, 3);
        assert_eq!(text.y, 3);
    }
}
