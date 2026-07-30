mod io;
mod metadata;
mod scandata;

use pyo3::prelude::*;

pub use io::NanoscopeFile;

#[pymodule]
mod rustyscope {

    use crate::io::NanoscopeFile;

    use numpy::PyArray1;
    use pyo3::{
        exceptions::{PyException, PyOSError},
        prelude::*,
    };

    type LinePair<'py> = (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>);

    #[pyfunction]
    fn load<'py>(py: Python<'py>, file_path: &str) -> PyResult<Vec<LinePair<'py>>> {
        let ns_file = NanoscopeFile::load(file_path)
            .map_err(|e| PyOSError::new_err(format!("Failed to load file: {e}")))?;

        let lines = ns_file.get_scan_lines().map_err(PyException::new_err)?;

        let mut py_lines = Vec::with_capacity(lines.len());

        for (x_vec, y_vec) in lines {
            let x_arr = PyArray1::from_vec(py, x_vec);
            let y_arr = PyArray1::from_vec(py, y_vec);

            py_lines.push((x_arr, y_arr));
        }

        Ok(py_lines)
    }
}
