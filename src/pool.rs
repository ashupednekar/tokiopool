use ::tokio::runtime::Runtime;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyFunction, PyTuple};
use tokio::time::{sleep, Duration};

#[pyclass]
pub struct TokioPoolExecutor {
    max_workers: i32,
    rt: Runtime,
}

#[pymethods]
impl TokioPoolExecutor {
    #[new]
    fn new(max_workers: i32) -> TokioPoolExecutor {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        TokioPoolExecutor { max_workers, rt }
    }

    #[pyo3(signature = (function, *args, **kwargs))]
    pub fn submit(
        &self,
        py: Python,
        function: Py<PyFunction>,
        args: Py<PyTuple>,
        kwargs: Option<Py<PyDict>>,
    ) {
        println!(
            "func: {:?}, args: {:?}, kwargs: {:?}",
            function, args, kwargs
        );
        py.allow_threads(move || {
            self.rt.block_on(async {
                let f = function.clone();
                tokio::spawn(async move {
                    println!("calling");
                    Python::with_gil(|py| {
                        let r = f.call(py, ("a",), None);
                        println!("r: {:?}", r);
                    });
                    println!("called");
                });
            });
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
