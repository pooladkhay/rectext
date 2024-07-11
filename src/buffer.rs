use crate::Error;

pub struct Buffer {
    width: usize,
    height: usize,
    inner: Vec<char>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            inner: vec![' '; width * height],
        }
    }

    pub fn clear(&mut self) {
        for cell in self.inner.iter_mut() {
            if *cell != ' ' {
                *cell = ' ';
            }
        }
    }

    pub fn draw_char(&mut self, x: usize, y: usize, c: char) -> Result<(), Error> {
        if x >= self.width || y >= self.height {
            return Err(Error::PositionError {
                x,
                y,
                width: self.width,
                height: self.height,
            });
        };

        let index = (y * self.width) + x;
        if self.inner[index] != c {
            self.inner[index] = c;
        };

        Ok(())
    }

    pub fn iter(&self) -> BufferIter {
        BufferIter {
            index: 0,
            inner: &self.inner,
        }
    }

    // Retrieve the buffer content, only for testing
    pub fn get_content(&self) -> Vec<char> {
        self.inner.clone()
    }
}

pub struct BufferIter<'a> {
    index: usize,
    inner: &'a Vec<char>,
}
impl<'a> Iterator for BufferIter<'a> {
    type Item = &'a char;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.get(self.index);
        self.index += 1;
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_initialization() {
        let buffer = Buffer::new(5, 3);
        let mut count = 0;
        for c in buffer.iter() {
            if *c == ' ' {
                count += 1
            }
        }
        assert_eq!(count, 15);
    }

    #[test]
    fn test_draw_char() -> Result<(), Error> {
        let mut buffer = Buffer::new(5, 3);
        buffer.draw_char(1, 1, 'X')?;
        assert_eq!(buffer.inner[6], 'X');
        Ok(())
    }

    #[test]
    fn test_clear_buffer() -> Result<(), Error> {
        let mut buffer = Buffer::new(10, 5);
        buffer.draw_char(1, 1, 'X')?;
        buffer.clear();
        for cell in buffer.inner.iter() {
            assert_eq!(*cell, ' ');
        }
        Ok(())
    }
}
