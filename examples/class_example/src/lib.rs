#![feature(specialization)]

#[macro_use]
extern crate pyo3;

use pyo3::types::PyType;
use pyo3::prelude::*;
use pyo3::PyRawObject;

/// A simple counter class with getter/setter properties
#[pyclass]
struct Counter {
    #[prop(get, set)]
    count: i32,
    name: String,
    token: PyToken,
}

#[pymethods]
impl Counter {
    #[new]
    fn __new__(obj: &PyRawObject, name: String) -> PyResult<()> {
        obj.init(|token| Counter {
            count: 0,
            name,
            token,
        })
    }

    /// Increment the counter by a given amount (default 1)
    #[args(amount = "1")]
    fn increment(&mut self, amount: i32) -> PyResult<i32> {
        self.count += amount;
        Ok(self.count)
    }

    /// Decrement the counter by a given amount (default 1)
    #[args(amount = "1")]
    fn decrement(&mut self, amount: i32) -> PyResult<i32> {
        self.count -= amount;
        Ok(self.count)
    }

    /// Reset the counter to zero
    fn reset(&mut self) -> PyResult<()> {
        self.count = 0;
        Ok(())
    }

    /// Get the counter name
    #[getter]
    fn get_name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    /// Create a counter with a specific initial value
    #[classmethod]
    fn with_value(cls: &PyType, name: String, value: i32) -> PyResult<Py<Counter>> {
        let py = unsafe { Python::assume_gil_acquired() };
        let counter = py.init(|token| Counter {
            count: value,
            name,
            token,
        })?;
        Ok(counter)
    }

    /// Get the maximum safe counter value
    #[staticmethod]
    fn max_value() -> PyResult<i32> {
        Ok(i32::max_value())
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Counter {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Counter(name='{}', count={})", self.name, self.count))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}: {}", self.name, self.count))
    }
}

/// A simple point class demonstrating comparison operators
#[pyclass]
struct Point {
    x: f64,
    y: f64,
    token: PyToken,
}

#[pymethods]
impl Point {
    #[new]
    fn __new__(obj: &PyRawObject, x: f64, y: f64) -> PyResult<()> {
        obj.init(|token| Point { x, y, token })
    }

    #[getter]
    fn x(&self) -> PyResult<f64> {
        Ok(self.x)
    }

    #[getter]
    fn y(&self) -> PyResult<f64> {
        Ok(self.y)
    }

    /// Calculate distance from origin
    fn distance_from_origin(&self) -> PyResult<f64> {
        Ok((self.x * self.x + self.y * self.y).sqrt())
    }

    /// Calculate distance to another point
    fn distance_to(&self, other: &Point) -> PyResult<f64> {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        Ok((dx * dx + dy * dy).sqrt())
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Point {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Point({}, {})", self.x, self.y))
    }

    fn __richcmp__(&self, other: &Point, op: pyo3::class::CompareOp) -> PyResult<bool> {
        use pyo3::class::CompareOp;

        let self_dist = self.distance_from_origin()?;
        let other_dist = other.distance_from_origin()?;

        match op {
            CompareOp::Lt => Ok(self_dist < other_dist),
            CompareOp::Le => Ok(self_dist <= other_dist),
            CompareOp::Eq => Ok((self.x - other.x).abs() < 1e-10 && (self.y - other.y).abs() < 1e-10),
            CompareOp::Ne => Ok((self.x - other.x).abs() >= 1e-10 || (self.y - other.y).abs() >= 1e-10),
            CompareOp::Gt => Ok(self_dist > other_dist),
            CompareOp::Ge => Ok(self_dist >= other_dist),
        }
    }
}

/// A number sequence iterator
#[pyclass]
struct Sequence {
    start: i32,
    end: i32,
    current: i32,
    token: PyToken,
}

#[pymethods]
impl Sequence {
    #[new]
    fn __new__(obj: &PyRawObject, start: i32, end: i32) -> PyResult<()> {
        obj.init(|token| Sequence {
            start,
            end,
            current: start,
            token,
        })
    }
}

#[pyproto]
impl pyo3::class::iter::PyIterProtocol for Sequence {
    fn __iter__(&mut self) -> PyResult<PyObject> {
        self.current = self.start;
        Ok(self.into())
    }

    fn __next__(&mut self) -> PyResult<Option<i32>> {
        if self.current < self.end {
            let value = self.current;
            self.current += 1;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

/// A Python module implemented in Rust demonstrating PyO3 classes.
#[pymodinit]
fn class_example(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Counter>()?;
    m.add_class::<Point>()?;
    m.add_class::<Sequence>()?;

    Ok(())
}
