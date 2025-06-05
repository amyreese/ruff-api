use pyo3::prelude::*;

use ruff_formatter::LineWidth;
use ruff_linter::linter::lint_fix;
use ruff_linter::registry::Rule;
use ruff_linter::rules::isort;
use ruff_linter::settings::{LinterSettings, flags, types::UnsafeFixes};
use ruff_linter::source_kind::SourceKind;
use ruff_python_ast::{self as ast, PySourceType};
use ruff_python_formatter::{FormatModuleError, PreviewMode, PyFormatOptions};

use glob::Pattern;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

pyo3::import_exception!(ruff_api.errors, FormatError);
pyo3::import_exception!(ruff_api.errors, ParseError);
pyo3::import_exception!(ruff_api.errors, PrintError);

/// Experimental Python API for Ruff
#[pymodule]
mod _rust {
    use super::*;

    // -- Formatting --

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
        fn new(
            target_version: Option<String>,
            line_width: Option<u16>,
            preview: Option<bool>,
        ) -> Self {
            Self {
                target_version: target_version
                    .unwrap_or(String::from("default"))
                    .to_lowercase(),
                line_width: line_width.unwrap_or(88),
                preview: preview.unwrap_or(false),
            }
        }
    }

    impl FormatOptions {
        fn to_format_options(&self, path: &Path) -> PyFormatOptions {
            PyFormatOptions::from_source_type(PySourceType::from(path))
                .with_target_version(match self.target_version.as_str() {
                    "py37" => ast::PythonVersion::PY37,
                    "py38" => ast::PythonVersion::PY38,
                    "py39" => ast::PythonVersion::PY39,
                    "py310" => ast::PythonVersion::PY310,
                    "py311" => ast::PythonVersion::PY311,
                    "py312" => ast::PythonVersion::PY312,
                    "py313" => ast::PythonVersion::PY313,
                    "py314" => ast::PythonVersion::PY314,
                    _ => ast::PythonVersion::default(),
                })
                .with_line_width(LineWidth::try_from(self.line_width).unwrap())
                .with_preview(match self.preview {
                    true => PreviewMode::Enabled,
                    false => PreviewMode::Disabled,
                })
        }
    }

    // -- Import Sorting --

    #[pyclass(get_all)]
    #[derive(Clone, Debug)]
    struct SortOptions {
        first_party_modules: Vec<String>,
        standard_library_modules: Vec<String>,
        case_sensitive: bool,
        combine_as_imports: bool,
        detect_same_package: bool,
        order_by_type: bool,
    }

    #[pymethods]
    impl SortOptions {
        #[new]
        #[pyo3(signature = (
            first_party_modules=None,
            standard_library_modules=None,
            case_sensitive=None,
            combine_as_imports=None,
            detect_same_package=None,
            order_by_type=None,
        ))]
        fn new(
            first_party_modules: Option<Vec<String>>,
            standard_library_modules: Option<Vec<String>>,
            case_sensitive: Option<bool>,
            combine_as_imports: Option<bool>,
            detect_same_package: Option<bool>,
            order_by_type: Option<bool>,
        ) -> Self {
            Self {
                first_party_modules: first_party_modules.unwrap_or_default(),
                standard_library_modules: standard_library_modules.unwrap_or_default(),
                // match default values from upstream ruff
                case_sensitive: case_sensitive.unwrap_or(false),
                combine_as_imports: combine_as_imports.unwrap_or(false),
                detect_same_package: detect_same_package.unwrap_or(true),
                order_by_type: order_by_type.unwrap_or(true),
            }
        }
    }

    impl Default for SortOptions {
        fn default() -> Self {
            Self {
                first_party_modules: vec![],
                standard_library_modules: vec![],
                // match default values from upstream ruff
                case_sensitive: false,
                combine_as_imports: false,
                detect_same_package: true,
                order_by_type: true,
            }
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
        /// handle converting from ruff's native errors to exported exceptions
        fn convert_error(error: &FormatModuleError) -> PyErr {
            match error {
                FormatModuleError::FormatError(err) => FormatError::new_err(err.to_string()),
                FormatModuleError::ParseError(err) => ParseError::new_err(err.to_string()),
                FormatModuleError::PrintError(err) => PrintError::new_err(err.to_string()),
            }
        }

        let path: &Path = Path::new(&path);
        let format_options: PyFormatOptions = match options {
            None => PyFormatOptions::default(),
            Some(options) => options.to_format_options(path),
        };
        match ruff_python_formatter::format_module_source(source.as_str(), format_options) {
            Ok(fm) => Ok(fm.into_code()),
            Err(e) => Err(convert_error(&e)),
        }
    }

    #[pyfunction]
    #[pyo3(signature = (path, source, options=None, root=None))]
    fn isort_string(
        path: String,
        source: String,
        options: Option<&SortOptions>,
        root: Option<String>,
    ) -> PyResult<String> {
        let ipath: &Path = Path::new(&path);

        let options: SortOptions = match options {
            None => SortOptions::default(),
            Some(options) => options.clone(),
        };

        let root_path = match root {
            None => env::current_dir()?,
            Some(value) => PathBuf::from(value),
        };

        let first_party_modules_pattern = options
            .first_party_modules
            .iter()
            .map(|s| Pattern::new(s).expect("Invalid pattern"))
            .collect();
        let standard_lib_modules_pattern = options
            .standard_library_modules
            .iter()
            .map(|s| Pattern::new(s).expect("Invalid pattern"))
            .collect();

        const CINDER_TOP_OF_FILE: &str = "cinder-top-of-file";

        let linter_settings: LinterSettings = LinterSettings {
            src: vec![root_path],
            isort: isort::settings::Settings {
                case_sensitive: options.case_sensitive,
                combine_as_imports: options.combine_as_imports,
                detect_same_package: options.detect_same_package,
                order_by_type: options.order_by_type,

                known_modules: isort::categorize::KnownModules::new(
                    first_party_modules_pattern,  // first-party
                    vec![],                       // third-party
                    vec![],                       // local
                    standard_lib_modules_pattern, // standard-lib
                    HashMap::from_iter([(
                        CINDER_TOP_OF_FILE.to_string(),
                        vec![
                            Pattern::new("__strict__").unwrap(),
                            Pattern::new("__static__").unwrap(),
                        ],
                    )]),
                ),
                section_order: vec![
                    isort::ImportSection::Known(isort::ImportType::Future),
                    isort::ImportSection::UserDefined(CINDER_TOP_OF_FILE.to_string()),
                    isort::ImportSection::Known(isort::ImportType::StandardLibrary),
                    isort::ImportSection::Known(isort::ImportType::ThirdParty),
                    isort::ImportSection::Known(isort::ImportType::FirstParty),
                    isort::ImportSection::Known(isort::ImportType::LocalFolder),
                ],
                ..Default::default()
            },
            ..LinterSettings::for_rules(vec![Rule::UnsortedImports])
        };

        let source_kind = match SourceKind::from_source_code(source, PySourceType::Python) {
            Ok(source_kind) => source_kind,
            Err(err) => {
                return Ok(err.to_string());
            }
        }
        .unwrap();

        let result = lint_fix(
            ipath,
            None,
            flags::Noqa::Enabled,
            UnsafeFixes::Disabled,
            &linter_settings,
            &source_kind,
            PySourceType::Python,
        );

        match result {
            Ok(diag) => Ok(diag.transformed.as_python().unwrap().to_string()),
            Err(error) => Err(PrintError::new_err(error.to_string())),
        }
    }
}
