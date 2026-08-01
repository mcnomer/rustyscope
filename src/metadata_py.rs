use crate::metadata::{Metadata, MetadataValue};
use pyo3::{prelude::*, types::PyDict};

#[pyclass(frozen, extends = PyDict, name="Metadata")]
pub struct PyMetadata {}

impl PyMetadata {
    pub fn from_metadata<'py>(
        py: Python<'py>,
        metadata: &Metadata,
    ) -> PyResult<Bound<'py, PyMetadata>> {
        let meta = Bound::new(py, PyMetadata {})?;
        let dict = meta.as_super();
        for (k, v) in &metadata.data {
            match v {
                MetadataValue::Integer(x) => dict.set_item(k, x)?,
                MetadataValue::Float(x) => dict.set_item(k, x)?,
                MetadataValue::String(s) => dict.set_item(k, s)?,
            };
        }
        Ok(meta)
    }

    pub fn from_metadata_option<'py>(
        py: Python<'py>,
        metadata_option: &Option<Metadata>,
    ) -> PyResult<Option<Bound<'py, PyMetadata>>> {
        match metadata_option {
            Some(metadata) => Ok(Some(PyMetadata::from_metadata(py, metadata)?)),
            None => Ok(None),
        }
    }
}
