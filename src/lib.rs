use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// This line assumes fib_calcs.rs is in the same directory and declared as a module
pub mod fib_calcs;

// Import the functions directly; no need for the __pyo3_get_function_ prefix
use fib_calcs::fib_number::fibonacci_number;
use fib_calcs::fib_numbers::fibonacci_numbers;

#[pyfunction]
fn say_hello() {
    println!("Saying hello from Rust!");
}

#[pymodule]
fn richard_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add the Rust functions to the Python module
    m.add_wrapped(wrap_pyfunction!(say_hello))?;
    m.add_wrapped(wrap_pyfunction!(fibonacci_number))?;
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers))?;
    Ok(())
}
