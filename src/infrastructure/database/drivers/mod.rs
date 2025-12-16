use crate::kernel::error::KernelError;

pub mod mysql;
pub mod postgres;
pub mod sqlite;

pub trait Driver {
  
    fn connect(&self) -> Result<(), KernelError>;
    fn execute(&self, query: &str) -> Result<(), KernelError>;
    fn transaction<F>(&self, f: F) -> Result<(), KernelError>
    where
        F: FnOnce() -> Result<(), KernelError>;
}
