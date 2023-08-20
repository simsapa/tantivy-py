use pyo3::{basic::CompareOp, prelude::*};
use tantivy as tv;

/// Tantivy schema.
///
/// The schema is very strict. To build the schema the `SchemaBuilder` class is
/// provided.
#[pyclass(frozen)]
#[derive(PartialEq)]
pub(crate) struct Schema {
    pub(crate) inner: tv::schema::Schema,
}

#[pymethods]
impl Schema {
    fn __richcmp__(
        &self,
        other: &Self,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn field_names(&self) -> PyResult<Vec<String>> {
        let mut names: Vec<String> = vec![];

        for v in self.inner.fields() {
            let (field, _) = v;
            let name = self.inner.get_field_name(field);
            names.push(name.to_string());
        }

        return Ok(names)
    }
}
