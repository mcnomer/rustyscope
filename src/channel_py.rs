use crate::channel::Channel;
use crate::metadata_py::PyMetadata;
use pyo3::prelude::*;

#[pyclass(frozen, name = "Channel")]
pub struct PyChannel {
    channel: Channel,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub data: Vec<i16>,
}

impl PyChannel {
    pub fn from_channel<'py>(
        py: Python<'py>,
        channel: &Channel,
    ) -> PyResult<Bound<'py, PyChannel>> {
        let chan = channel.clone();
        let name = chan.name.to_owned();
        let data = chan.data.clone();
        Bound::new(
            py,
            PyChannel {
                channel: chan,
                name,
                data,
            },
        )
    }
}

#[pymethods]
impl PyChannel {
    #[getter]
    pub fn get_metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyMetadata>> {
        PyMetadata::from_metadata(py, &self.channel.metadata)
    }
}
