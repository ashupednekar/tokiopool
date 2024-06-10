use ::tokio::runtime::Runtime;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyFunction, PyTuple};

#[pyclass]
pub struct TokioPoolExecutor {
    max_workers: i32,
    rt: Runtime,
}

#[pymethods]
impl TokioPoolExecutor {
    #[new]
    fn new(max_workers: i32) -> TokioPoolExecutor {
        TokioPoolExecutor {
            max_workers,
            rt: Runtime::new().unwrap(),
        }
    }

    #[pyo3(signature = (function, *py_args, **py_kwargs))]
    pub fn submit(
        &self,
        py: Python,
        function: Py<PyFunction>,
        py_args: &PyTuple,
        py_kwargs: Option<&PyDict>,
    ) {
        println!(
            "func: {:?}, args: {:?}, kwargs: {:?}",
            function, py_args, py_kwargs
        );
        py.allow_threads(move || {
            self.rt.block_on(tokio::spawn(async move {
                Python::with_gil(|py| {
                    function.call(py, ("a",), None);
                });
            }));
        });
    }

    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __exit__(
        &self,
        _py: Python,
        _exc_type: Option<&PyAny>,
        _exc_value: Option<&PyAny>,
        _traceback: Option<&PyAny>,
    ) {
        // Perform any necessary cleanup here
    }
}
