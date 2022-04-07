use std::fmt::{Display, Formatter};
use winapi::shared::winerror::{E_NOINTERFACE, E_POINTER};

#[derive(Debug)]
pub enum AppError {
    DeviceCreateFailed,
    NoInterface,
    Pointer,

    Unknonw,
}

impl Display for AppError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(formatter, "{:?}", self)
    }
}

impl From<i32> for AppError {
    fn from(error: i32) -> Self {
        match error {
            E_NOINTERFACE => AppError::NoInterface,
            E_POINTER => AppError::Pointer,
            _ => AppError::Unknonw,
        }
    }
}
