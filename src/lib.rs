use pyo3::prelude::*;

use rayon::str::ParallelString;
use rayon::iter::ParallelIterator;

fn count_line(line: &str, needle: &str) -> usize {
    let mut total = 0;
    for word in line.split(' ') {
        if word == needle {
            total += 1;
        }
    }
    total
}

#[pyfunction]
fn search(contents: &str, needle: &str) -> usize {
    contents
        .par_lines()
        .map(|line| count_line(line, needle))
        .sum()
}

#[pyfunction]
fn search_sequential(contents: &str, needle: &str) -> usize {
    contents.lines().map(|line| count_line(line, needle)).sum()
}

#[pyfunction]
fn search_sequential_allow_threads(py: Python<'_>, contents: &str, needle: &str) -> usize {
    py.allow_threads(|| search_sequential(contents, needle))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn sum_as_int(a: usize, b: usize) -> PyResult<usize> {
    Ok(a + b)
}


#[pyclass]
struct Number(i32);

#[pymethods]
impl Number {
    #[new]
    fn  new(value: i32) -> Self {
        Number(value)
    }

    fn value(&self) -> i32 {
        self.0
    }

    fn add(&self, other: &Number) -> PyResult<Number> {
        Ok(Number(self.0 + other.0))
    }

    fn sub(&self, other: &Number) -> PyResult<Number> {
        Ok(Number(self.0 - other.0))
    }

    fn mul(&self, other: &Number) -> PyResult<Number> {
        Ok(Number(self.0 * other.0))
    }

    fn div(&self, other: &Number) -> PyResult<Number> {
        Ok(Number(self.0 / other.0))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn RustPythonBindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(search, m)?)?;
    m.add_function(wrap_pyfunction!(search_sequential, m)?)?;
    m.add_function(wrap_pyfunction!(search_sequential_allow_threads, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_int, m)?)?;
    m.add_class::<Number>()?;
    Ok(())
}
