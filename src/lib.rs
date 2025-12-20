use pyo3::prelude::*;

pub mod kernel;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
#[pymodule]
mod tikal {
    use pyo3::prelude::*;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

}