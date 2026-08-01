mod channel;
mod channel_py;
mod io;
mod metadata;
mod metadata_py;

use numpy::PyArray1;
use pyo3::{exceptions::PyOSError, prelude::*, types::*};

pub use io::NanoscopeFile;

use crate::{channel_py::PyChannel, metadata_py::PyMetadata};

#[pymodule]
mod rustyscope {
    #[pymodule_export]
    use crate::AFPFile;

    #[pymodule_export]
    use crate::metadata_py::PyMetadata;

    #[pymodule_export]
    use crate::channel_py::PyChannel;
}

type LinePair<'py> = (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>);

#[pyclass(frozen)]
pub struct AFPFile {
    file: NanoscopeFile,
    #[pyo3(get)]
    pub file_path: String,
}

#[pymethods]
impl AFPFile {
    #[new]
    pub fn __init__(file_path: Bound<'_, PyString>) -> PyResult<Self> {
        let path = &file_path.to_string();
        let ns_file = NanoscopeFile::load(path)
            .map_err(|e| PyOSError::new_err(format!("Failed to load file: {e}")))?;
        Ok(Self {
            file: ns_file,
            file_path: file_path.to_string(),
        })
    }

    #[getter]
    pub fn get_data<'py>(&self, py: Python<'py>) -> PyResult<Vec<LinePair<'py>>> {
        let lines = self.file.data.clone();
        let mut py_lines = Vec::with_capacity(lines.len());

        for (x_vec, y_vec) in lines {
            let x_arr = PyArray1::from_vec(py, x_vec);
            let y_arr = PyArray1::from_vec(py, y_vec);

            py_lines.push((x_arr, y_arr));
        }

        Ok(py_lines)
    }

    #[getter]
    pub fn get_channels<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyChannel>>> {
        self.file
            .channels
            .iter()
            .map(|chan| PyChannel::from_channel(py, chan))
            .collect()
    }

    #[getter]
    pub fn get_file_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyMetadata>> {
        PyMetadata::from_metadata(py, &self.file.file_metadata)
    }

    #[getter]
    pub fn get_scanner_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyMetadata>> {
        PyMetadata::from_metadata(py, &self.file.scanner_metadata)
    }

    #[getter]
    pub fn get_equipment_metadata<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyMetadata>>> {
        PyMetadata::from_metadata_option(py, &self.file.equipment_metadata)
    }

    #[getter]
    pub fn get_hdsc_metadata<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyMetadata>>> {
        PyMetadata::from_metadata_option(py, &self.file.hdsc_metadata)
    }

    #[getter]
    pub fn get_misc_metadata<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyMetadata>>> {
        PyMetadata::from_metadata_option(py, &self.file.misc_metadata)
    }

    #[getter]
    pub fn get_engage_metadata<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyMetadata>>> {
        PyMetadata::from_metadata_option(py, &self.file.engage_metadata)
    }

    #[getter]
    pub fn get_sweep_metadata<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyMetadata>>> {
        PyMetadata::from_metadata_option(py, &self.file.sweep_metadata)
    }
}
