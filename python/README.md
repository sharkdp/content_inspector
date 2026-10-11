# content_inspector (Python)

Python bindings (PyO3 + maturin) for [`content_inspector`](..): fast inspection of binary buffers
to guess whether they contain text or binary data.

```bash
pip install maturin
cd python
maturin develop --release
```

```python
import content_inspector as ci

t = ci.inspect(b"hello")       # bytes, bytearray, memoryview, ...
t.is_binary()                  # False
t.is_text()                    # True
t.name                         # "UTF_8"

ci.inspect("some/file")        # a str or os.PathLike is read from disk
ci.inspect(b"\x00\x01").is_binary()  # True
```

`inspect` returns a `ContentType`, which compares and hashes by value and prints as e.g.
`ContentType.UTF_16LE`. Possible names: `BINARY`, `UTF_8`, `UTF_8_BOM`, `UTF_16LE`, `UTF_16BE`,
`UTF_32LE`, `UTF_32BE`.

Only the first 1024 bytes are examined, exactly as in the Rust crate; see the
[main README](../README.md) for the limits of this heuristic.
