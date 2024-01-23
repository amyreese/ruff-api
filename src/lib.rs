use pyo3::exceptions::{self};
use pyo3::{create_exception, prelude::*};
use ruff;
use ruff_formatter::LineWidth;
use ruff_python_ast::PySourceType;
use ruff_python_formatter::{self, PreviewMode, PyFormatOptions, PythonVersion};
use std::path::Path;

create_exception!(ruff_api, FormatModuleError, exceptions::PyException);
create_exception!(ruff_api, FormatError, FormatModuleError);
create_exception!(ruff_api, ParseError, FormatModuleError);
create_exception!(ruff_api, PrintError, FormatModuleError);

// handle converting from ruff's native errors to exported exceptions
fn convert_error(error: &ruff_python_formatter::FormatModuleError) -> PyErr {
    match error {
        ruff_python_formatter::FormatModuleError::FormatError(e) => {
            FormatError::new_err(e.to_string())
        }
        ruff_python_formatter::FormatModuleError::ParseError(e) => {
            ParseError::new_err(e.to_string())
        }
        ruff_python_formatter::FormatModuleError::PrintError(e) => {
            PrintError::new_err(e.to_string())
        }
    }
}

#[pyclass(get_all)]
struct FormatOptions {
    target_version: String,
    line_width: u16,
    preview: bool,
}

#[pymethods]
impl FormatOptions {
    #[new]
    #[pyo3(signature = (target_version=None, line_width=None, preview=None))]
    fn new(target_version: Option<String>, line_width: Option<u16>, preview: Option<bool>) -> Self {
        Self {
            target_version: target_version.unwrap_or(String::from("default")).to_lowercase(),
            line_width: line_width.unwrap_or(88),
            preview: preview.unwrap_or(false),
        }
    }
}

impl FormatOptions {
    fn to_format_options(&self, path: &Path) -> PyFormatOptions {
        PyFormatOptions::from_source_type(PySourceType::from(path))
            .with_target_version(match self.target_version.as_str() {
                "py37" => PythonVersion::Py37,
                "py38" => PythonVersion::Py38,
                "py39" => PythonVersion::Py39,
                "py310" => PythonVersion::Py310,
                "py311" => PythonVersion::Py311,
                "py312" => PythonVersion::Py312,
                _ => PythonVersion::default(),
            })
            .with_line_width(LineWidth::try_from(self.line_width).unwrap())
            .with_preview(match self.preview {
                true => PreviewMode::Enabled,
                false => PreviewMode::Disabled,
            })
    }
}

/// Formats a string of code with the given options
#[pyfunction]
#[pyo3(signature = (path, source, options=None))]
fn format_string(
    path: String,
    source: String,
    options: Option<&FormatOptions>,
) -> PyResult<String> {
    let path: &Path = Path::new(&path);
    let format_options: PyFormatOptions = match options {
        None => PyFormatOptions::default(),
        Some(options) => options.to_format_options(&path),
    };
    match ruff_python_formatter::format_module_source(&source.as_str(), format_options) {
        Ok(fm) => Ok(fm.into_code()),
        Err(e) => Err(convert_error(&e)),
    }
}

/// Experimental Python API for Ruff
#[pymodule]
#[pyo3(name = "_rust")]
fn ruff_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format_string, m)?)?;
    m.add_class::<FormatOptions>()?;
    m.add("FormatModuleError", _py.get_type::<FormatModuleError>())?;
    m.add("FormatError", _py.get_type::<FormatError>())?;
    m.add("ParseError", _py.get_type::<ParseError>())?;
    m.add("PrintError", _py.get_type::<PrintError>())?;
    Ok(())
}
