use super::Driver;
use crate::kernel::error::KernelError;
pub struct SQLiteDriver;

impl Driver for SQLiteDriver {
    fn connect(&self) -> Result<(), KernelError> {
    }

    fn execute(&self, query: &str) -> Result<(), KernelError> {
    }

    fn transaction<F>(&self, f: F) -> Result<(), KernelError>
    where
        F: FnOnce() -> Result<(), KernelError>,
    {

    }
}