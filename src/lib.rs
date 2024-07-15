pub mod buffer;
pub mod elements;
pub mod error;
pub mod rectext;
pub mod traits;

pub use buffer::Buffer;
pub use elements::{Container, Rectangle, Text};
pub use error::Error;
pub use rectext::Rectext;
pub use terminal::TerminalCommand;
pub use traits::UIElement;

mod terminal;
use terminal::Terminal;
