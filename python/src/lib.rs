use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use content_inspector::ContentType;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PySlice, PyString};

/// content_inspector only looks at the first 1024 bytes, so reading more is wasted I/O.
const SCAN: usize = 1024;

fn type_name(t: ContentType) -> &'static str {
    match t {
        ContentType::BINARY => "BINARY",
        ContentType::UTF_8 => "UTF_8",
        ContentType::UTF_8_BOM => "UTF_8_BOM",
        ContentType::UTF_16LE => "UTF_16LE",
        ContentType::UTF_16BE => "UTF_16BE",
        ContentType::UTF_32LE => "UTF_32LE",
        ContentType::UTF_32BE => "UTF_32BE",
    }
}

// `Read::read` may return fewer bytes than requested even when more data remains
// (pipes, network filesystems, signal interruptions), so we loop until we have
// SCAN bytes or hit EOF. `n` tracks how many bytes we've read so far, and each
// read picks up at `buf[n..]`.
fn read_prefix(path: &PathBuf, buf: &mut [u8; SCAN]) -> std::io::Result<usize> {
    let mut f = File::open(path)?;
    let mut n = 0;
    while n < SCAN {
        match f.read(&mut buf[n..]) {
            Ok(0) => break,
            Ok(k) => n += k,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(n)
}

fn inspect_file(path: &PathBuf) -> std::io::Result<ContentType> {
    let mut buf = [0u8; SCAN];
    let n = read_prefix(path, &mut buf)?;
    Ok(content_inspector::inspect(&buf[..n]))
}

/// The first `SCAN` bytes of any object supporting the buffer protocol (`bytes`, `bytearray`,
/// `memoryview`, ...).
///
/// PyO3's `PyBuffer` is only available under abi3 from Python 3.11, so anything that is not
/// `bytes` goes through a Python `memoryview`; only the first `SCAN` bytes are ever copied.
fn prefix_of(data: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    if let Ok(b) = data.cast::<PyBytes>() {
        let b = b.as_bytes();
        return Ok(b[..b.len().min(SCAN)].to_vec());
    }

    let py = data.py();
    let builtins = py.import("builtins")?;
    let view = builtins.getattr("memoryview")?.call1((data,))?;
    let flat = view.call_method1("cast", ("B",))?;
    let head = flat.get_item(PySlice::new(py, 0, SCAN as isize, 1))?;
    builtins.getattr("bytes")?.call1((head,))?.extract()
}

/// The kind of content found: `BINARY` or a text encoding.
#[pyclass(
    name = "ContentType",
    frozen,
    eq,
    skip_from_py_object,
    module = "content_inspector"
)]
#[derive(Clone, Copy, PartialEq)]
struct PyContentType(ContentType);

#[pymethods]
impl PyContentType {
    /// Name of the detected type: "BINARY", "UTF_8", "UTF_8_BOM", "UTF_16LE", ...
    #[getter]
    fn name(&self) -> &'static str {
        type_name(self.0)
    }

    /// True if the content looks like binary data.
    fn is_binary(&self) -> bool {
        self.0.is_binary()
    }

    /// True if the content looks like text.
    fn is_text(&self) -> bool {
        self.0.is_text()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("ContentType.{}", type_name(self.0))
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        type_name(self.0).hash(&mut h);
        h.finish()
    }
}

/// Guess the type of content in a file or an in-memory buffer.
///
/// `source` is a path (`str` or `os.PathLike`) or a bytes-like object (`bytes`, `bytearray`,
/// `memoryview`, ...). Only the first 1024 bytes are examined. Returns a `ContentType`:
///
///     inspect(b"hello").is_binary()      # False
///     inspect("some/file").name          # "UTF_8"
#[pyfunction]
fn inspect(py: Python<'_>, source: &Bound<'_, PyAny>) -> PyResult<PyContentType> {
    if source.is_instance_of::<PyString>() || source.hasattr("__fspath__")? {
        let path: PathBuf = source.extract()?;
        return Ok(PyContentType(py.detach(|| inspect_file(&path))?));
    }
    Ok(PyContentType(content_inspector::inspect(&prefix_of(
        source,
    )?)))
}

#[pymodule]
#[pyo3(name = "content_inspector")]
fn content_inspector_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyContentType>()?;
    m.add_function(wrap_pyfunction!(inspect, m)?)?;
    Ok(())
}
