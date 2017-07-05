use gfx;

use std::fmt;
use std::error;

pub type RenderResult<T> = Result<T, RenderError>;

#[derive(Debug)]
pub enum RenderError {
    BufferCreation(gfx::buffer::CreationError),
    NoSuchTarget(String),
    ProgramCreation(gfx::shade::ProgramError)
}

impl error::Error for RenderError {
    fn description(&self) -> &str {
        match *self {
            RenderError::BufferCreation(_) => "Failed to create buffer.",
            RenderError::NoSuchTarget(_) => "Target with this name does not exist.",
            RenderError::ProgramCreation(_) => "Failed to create shader program."
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RenderError::BufferCreation(ref e) => Some(e),
            RenderError::ProgramCreation(ref e) => Some(e),
            _ => None
        }
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderError::BufferCreation(ref e) => write!(fmt, "Buffer creation failed: {}", e),
            RenderError::NoSuchTarget(ref e) => write!(fmt, "Nonexistent target: {}", e),
            RenderError::ProgramCreation(ref e) => write!(fmt, "Program compilation failed: {}", e),
        }
    }
}
