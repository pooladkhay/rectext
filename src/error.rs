use std::{
    fmt::{Debug, Display},
    io,
};

use nix::errno;

pub enum Error {
    Errno(errno::Errno),
    IoError(io::Error),
    PositionError {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    },
}

impl From<errno::Errno> for Error {
    fn from(value: errno::Errno) -> Self {
        Self::Errno(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "IO Error: {}", err),
            Error::PositionError{ x, y, width, height } => write!(f, "Position Error: 'x' must be less than the buffer's width ({}) and 'y' must be less than the buffer's height ({}). Given: x = {}, y = {}", width, height, x, y),
            Error::Errno(ref err) => write!(f, "Errno: {}", err),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Errno(arg0) => f.debug_tuple("IoError").field(arg0).finish(),
            Self::IoError(arg0) => f.debug_tuple("IoError").field(arg0).finish(),
            Self::PositionError {
                x,
                y,
                width,
                height,
            } => f
                .debug_struct("PositionError")
                .field("x", x)
                .field("y", y)
                .field("width", width)
                .field("height", height)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_from_errno() {
        let errno = errno::Errno::EACCES;
        let error: Error = errno.into();
        if let Error::Errno(e) = error {
            assert_eq!(e, errno::Errno::EACCES);
        } else {
            panic!("Expected Error::Errno");
        }
    }

    #[test]
    fn test_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let error: Error = io_error.into();
        if let Error::IoError(e) = error {
            assert_eq!(e.kind(), io::ErrorKind::NotFound);
            assert_eq!(e.to_string(), "File not found");
        } else {
            panic!("Expected Error::IoError");
        }
    }

    #[test]
    fn test_display_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let error = Error::IoError(io_error);
        assert_eq!(format!("{}", error), "IO Error: File not found");
    }

    #[test]
    fn test_display_position_error() {
        let error = Error::PositionError {
            x: 10,
            y: 20,
            width: 30,
            height: 40,
        };
        assert_eq!(
            format!("{}", error),
            "Position Error: 'x' must be less than the buffer's width (30) and 'y' must be less than the buffer's height (40). Given: x = 10, y = 20"
        );
    }

    #[test]
    fn test_display_errno() {
        let errno = errno::Errno::EACCES;
        let error = Error::Errno(errno);
        assert_eq!(format!("{}", error), format!("Errno: {}", errno));
    }

    #[test]
    fn test_debug_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let error = Error::IoError(io_error);
        assert_eq!(
            format!("{:?}", error),
            format!(
                "IoError({:?})",
                io::Error::new(io::ErrorKind::NotFound, "File not found")
            )
        );
    }

    #[test]
    fn test_debug_position_error() {
        let error = Error::PositionError {
            x: 10,
            y: 20,
            width: 30,
            height: 40,
        };
        assert_eq!(
            format!("{:?}", error),
            "PositionError { x: 10, y: 20, width: 30, height: 40 }"
        );
    }

    #[test]
    fn test_debug_errno() {
        let errno = errno::Errno::EACCES;
        let error = Error::Errno(errno);
        assert_eq!(format!("{:?}", error), format!("IoError({:?})", errno));
    }
}
