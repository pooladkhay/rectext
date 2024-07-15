use std::{
    io::{self, Read, Write},
    os::fd::AsRawFd,
};

use nix::fcntl::{
    fcntl,
    FcntlArg::{F_GETFL, F_SETFL},
    OFlag,
};
use termios::Termios;

use crate::Error;

pub enum TerminalCommand<'a> {
    HideCursor,
    UnhideCursor,
    ClearScreen,
    PrintStr(&'a str),
    PrintChar(char),
    ///(x, y)
    MoveCursorTo(usize, usize),
}

pub struct Terminal<T, U>
where
    T: AsRawFd + Read,
    U: AsRawFd + Write,
{
    width: usize,
    height: usize,
    local_buffer: String,
    stdin: T,
    stdout: U,
}

impl<T, U> Terminal<T, U>
where
    T: AsRawFd + Read,
    U: AsRawFd + Write,
{
    pub fn new(width: usize, height: usize, stdin: T, stdout: U) -> Self {
        Self {
            width,
            height,
            local_buffer: String::new(),
            stdin,
            stdout,
        }
    }

    /// Reads one byte at a time from the stdin
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        let mut buf = [0_u8; 1];

        match self.stdin.read(&mut buf) {
            Ok(n) if n > 0 => Ok(buf[0]),
            Ok(_) => Err(Error::IoError(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "End of input or no more data available.",
            ))),
            Err(err) => Err(Error::IoError(io::Error::new(err.kind(), err))),
        }
    }

    pub fn raw_mode(&mut self, enable: bool) -> Result<(), Error> {
        let mut termios = Termios::from_fd(self.stdin.as_raw_fd())?;

        if enable {
            termios.c_lflag &= !termios::ECHO;
            termios.c_lflag &= !termios::ICANON;
        } else {
            termios.c_lflag |= termios::ECHO;
            termios.c_lflag |= termios::ICANON;
        }

        termios::tcsetattr(self.stdin.as_raw_fd(), termios::TCSANOW, &mut termios)?;

        Ok(())
    }

    /// Enables/Disables the `O_NONBLOCK` flag on the stdin file descriptor.
    /// More information: [man fcntl(2)](https://man7.org/linux/man-pages/man2/fcntl.2.html)
    pub fn stdin_non_blocking(&self, enable: bool) -> Result<(), Error> {
        let stdin_fd = self.stdin.as_raw_fd();
        let curr_flags = fcntl(stdin_fd, F_GETFL)?;

        let new_flags: OFlag;
        if enable {
            new_flags = OFlag::from_bits_truncate(curr_flags) | OFlag::O_NONBLOCK;
        } else {
            new_flags = OFlag::from_bits_truncate(curr_flags) & !OFlag::O_NONBLOCK;
        }

        fcntl(stdin_fd, F_SETFL(new_flags))?;

        Ok(())
    }

    /// Flushes the current buffer to the stdout of the process.
    pub fn flush(&mut self) -> Result<(), Error> {
        write!(self.stdout, "{}", self.local_buffer)?;
        self.stdout.flush()?;
        self.local_buffer.clear();
        Ok(())
    }

    /// Immediately executes a given ANSI command, without modifying the display buffer.
    pub fn exec_cmd(&self, command: TerminalCommand) -> Result<(), Error> {
        match command {
            TerminalCommand::HideCursor => todo!(),
            TerminalCommand::UnhideCursor => todo!(),
            TerminalCommand::ClearScreen => todo!(),
            TerminalCommand::PrintStr(_) => todo!(),
            TerminalCommand::PrintChar(_) => todo!(),
            TerminalCommand::MoveCursorTo(_, _) => todo!(),
        };
        // Ok(())
    }

    /// Adds a given ANSI command to the screen buffer to be executed later.
    pub fn buffer_cmd(&mut self, command: TerminalCommand) -> Result<(), Error> {
        match command {
            TerminalCommand::HideCursor => self.local_buffer.push_str("\x1b[?25l"),
            TerminalCommand::UnhideCursor => self.local_buffer.push_str("\x1b[?25h"),
            TerminalCommand::ClearScreen => self.local_buffer.push_str("\x1b[2J"),
            TerminalCommand::PrintStr(text) => self.local_buffer.push_str(text),
            TerminalCommand::PrintChar(ch) => self.local_buffer.push(ch),
            TerminalCommand::MoveCursorTo(x, y) => {
                if x > self.width || y > self.height {
                    return Err(Error::PositionError {
                        x,
                        y,
                        width: self.width,
                        height: self.height,
                    });
                };
                self.local_buffer
                    .push_str(format!("\x1b[{};{}H", y, x).as_str())
            }
        };
        Ok(())
    }
}
