use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyclass(module = "rusty_math")]
#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
struct Vec3 {
    #[pyo3(get, set)]
    x: f64,
    #[pyo3(get, set)]
    y: f64,
    #[pyo3(get, set)]
    z: f64,
}

#[gen_stub_pymethods]
#[pymethods]
impl Vec3 {
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> PyResult<Vec3> {
        let mag = self.magnitude();
        if mag == 0.0 {
            Err(pyo3::exceptions::PyValueError::new_err("cannot normalize zero vector"))
        } else {
            Ok(Vec3 { x: self.x / mag, y: self.y / mag, z: self.z / mag })
        }
    }

    fn angle(&self, other: &Vec3) -> PyResult<f64> {
        let mag_product = self.magnitude() * other.magnitude();
        if mag_product == 0.0 {
            Err(pyo3::exceptions::PyValueError::new_err("cannot compute angle with zero vector"))
        } else {
            Ok((self.dot(other) / mag_product).clamp(-1.0, 1.0).acos())
        }
    }

    fn __repr__(&self) -> String {
        format!("Vec3({}, {}, {})", self.x, self.y, self.z)
    }

    fn __add__(&self, other: &Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }

    fn __sub__(&self, other: &Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }

    fn __mul__(&self, scalar: f64) -> Vec3 {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

#[pymodule]
mod rusty_math {
    use pyo3::prelude::*;
    use pyo3_stub_gen::derive::*;

    #[pymodule_export]
    use crate::Vec3;

    #[gen_stub_pyfunction(module = "rusty_math")]
    #[pyfunction]
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    #[gen_stub_pyfunction(module = "rusty_math")]
    #[pyfunction]
    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    #[gen_stub_pyfunction(module = "rusty_math")]
    #[pyfunction]
    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    #[gen_stub_pyfunction(module = "rusty_math")]
    #[pyfunction]
    fn divide(a: f64, b: f64) -> PyResult<f64> {
        if b == 0.0 {
            Err(pyo3::exceptions::PyZeroDivisionError::new_err("division by zero"))
        } else {
            Ok(a / b)
        }
    }
}

define_stub_info_gatherer!(stub_info);
