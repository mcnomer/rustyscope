mod io;
mod metadata;
mod scandata;

use numpy::PyArray1;
use pyo3::{
    exceptions::{PyException, PyOSError},
    prelude::*,
    types::*,
};

pub use io::NanoscopeFile;

use crate::metadata::{Metadata, MetadataValue};

#[pymodule]
mod rustyscope {
    #[pymodule_export]
    use crate::AFPFile;
}

type LinePair<'py> = (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>);

#[pyclass(frozen)]
pub struct AFPFile {
    file: NanoscopeFile,
}

#[pymethods]
impl AFPFile {
    #[new]
    pub fn __init__(file_path: Bound<'_, PyString>) -> PyResult<Self> {
        let path = &file_path.to_string();
        let ns_file = NanoscopeFile::load(path)
            .map_err(|e| PyOSError::new_err(format!("Failed to load file: {e}")))?;
        Ok(Self { file: ns_file })
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
    pub fn get_file_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metadata_to_dict(py, &self.file.file_metadata)
    }

    #[getter]
    pub fn get_scanner_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metadata_to_dict(py, &self.file.scanner_metadata)
    }

    pub fn get_equipment_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metadata_option_to_dict(py, &self.file.equipment_metadata)
    }
    pub fn get_hdsc_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metadata_option_to_dict(py, &self.file.hdsc_metadata)
    }
    pub fn get_misc_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metadata_option_to_dict(py, &self.file.misc_metadata)
    }
    pub fn get_engage_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metadata_option_to_dict(py, &self.file.engage_metadata)
    }
    pub fn get_sweep_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metadata_option_to_dict(py, &self.file.sweep_metadata)
    }
}

fn metadata_to_dict<'py>(py: Python<'py>, metadata: &Metadata) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (k, v) in &metadata.data {
        match v {
            MetadataValue::Integer(x) => dict.set_item(k, x)?,
            MetadataValue::Float(x) => dict.set_item(k, x)?,
            MetadataValue::String(s) => dict.set_item(k, s)?,
        };
    }
    Ok(dict)
}

fn metadata_option_to_dict<'py>(
    py: Python<'py>,
    metadata_option: &Option<Metadata>,
) -> PyResult<Bound<'py, PyDict>> {
    match metadata_option {
        Some(metadata) => metadata_to_dict(py, metadata),
        None => Err(PyException::new_err(
            "Rustyscope error: dictionary not found in file.",
        )),
    }
}
