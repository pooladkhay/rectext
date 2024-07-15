use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, Write},
    os::fd::AsRawFd,
    rc::Rc,
};

use crate::{Buffer, Error, Terminal, TerminalCommand, UIElement};

pub struct Rectext<T, U>
where
    T: AsRawFd + Read,
    U: AsRawFd + Write,
{
    width: usize,
    height: usize,
    front_buffer: Buffer,
    back_buffer: Buffer,
    elements: HashMap<String, Rc<RefCell<dyn UIElement>>>,
    pub terminal: Terminal<T, U>,
}

impl<T, U> Rectext<T, U>
where
    T: AsRawFd + Read,
    U: AsRawFd + Write,
{
    pub fn new(width: usize, height: usize, stdin: T, stdout: U) -> Self {
        let terminal = Terminal::new(width, height, stdin, stdout);
        Self {
            width,
            height,
            front_buffer: Buffer::new(width, height),
            back_buffer: Buffer::new(width, height),
            elements: HashMap::new(),
            terminal,
        }
    }

    pub fn add_element(&mut self, id: String, element: Rc<RefCell<dyn UIElement>>) {
        self.elements.insert(id, element);
    }

    pub fn remove_element(&mut self, id: &str) {
        self.elements.remove(id);
    }

    pub fn draw(&mut self) -> Result<(), Error> {
        self.back_buffer.clear();

        for element in self.elements.values() {
            element.borrow().draw(&mut self.back_buffer)?;
        }

        self.render()?;

        self.terminal.flush()?;

        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);

        Ok(())
    }

    fn render(&mut self) -> Result<(), Error> {
        let mut gaps = 0;
        let mut first_gap = 0;

        self.terminal
            .buffer_cmd(TerminalCommand::MoveCursorTo(1, 1))?;

        for ((i, b_cell), f_cell) in self
            .back_buffer
            .iter()
            .enumerate()
            .zip(self.front_buffer.iter())
        {
            if *b_cell == *f_cell {
                if gaps == 0 {
                    first_gap = i;
                }
                gaps += 1;
            } else {
                if gaps != 0 {
                    // Column
                    let x = ((first_gap + gaps) % self.width) + 1;
                    // Row
                    let y = ((first_gap + gaps) / self.width) + 1;
                    self.terminal
                        .buffer_cmd(TerminalCommand::MoveCursorTo(x, y))?;
                    gaps = 0;
                    first_gap = 0;
                }
                self.terminal
                    .buffer_cmd(TerminalCommand::PrintChar(*b_cell))?;
            }
        }

        Ok(())
    }
}
