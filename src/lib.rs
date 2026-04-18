use pyo3::prelude::*;

#[pymodule]
mod rusty_math {
    use pyo3::prelude::*;

    #[pyfunction]
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    #[pyfunction]
    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    #[pyfunction]
    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    #[pyfunction]
    fn divide(a: f64, b: f64) -> PyResult<f64> {
        if b == 0.0 {
            Err(pyo3::exceptions::PyZeroDivisionError::new_err("division by zero"))
        } else {
            Ok(a / b)
        }
    }
}
