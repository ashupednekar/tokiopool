use pyo3::prelude::*;

mod pool;

#[pymodule]
fn tokiopool(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<pool::TokioPoolExecutor>()?;
    Ok(())
}
